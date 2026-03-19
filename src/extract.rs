use pac::ast::*;
use pac::span::Node;

use crate::diagnostics::Diagnostic;
use crate::ir::*;

pub struct Extractor {
    pub items: Vec<BindingItem>,
    pub diagnostics: Vec<Diagnostic>,
}

impl Extractor {
    pub fn new() -> Self {
        Self {
            items: Vec::new(),
            diagnostics: Vec::new(),
        }
    }

    pub fn extract(mut self, unit: &TranslationUnit) -> (Vec<BindingItem>, Vec<Diagnostic>) {
        for ext_decl in &unit.0 {
            self.extract_external_declaration(ext_decl);
        }
        (self.items, self.diagnostics)
    }

    fn extract_external_declaration(&mut self, ext_decl: &Node<ExternalDeclaration>) {
        match &ext_decl.node {
            ExternalDeclaration::Declaration(decl) => {
                self.extract_declaration(&decl.node, decl.span.start);
            }
            ExternalDeclaration::FunctionDefinition(fdef) => {
                self.extract_function_definition(&fdef.node, fdef.span.start);
            }
            ExternalDeclaration::StaticAssert(_) => {}
        }
    }

    fn extract_declaration(&mut self, decl: &Declaration, offset: usize) {
        let is_typedef = decl.specifiers.iter().any(|s| {
            matches!(
                s.node,
                DeclarationSpecifier::StorageClass(ref sc)
                    if sc.node == StorageClassSpecifier::Typedef
            )
        });

        let is_extern = decl.specifiers.iter().any(|s| {
            matches!(
                s.node,
                DeclarationSpecifier::StorageClass(ref sc)
                    if sc.node == StorageClassSpecifier::Extern
            )
        });

        // Extract standalone struct/union/enum definitions (no declarators)
        if decl.declarators.is_empty() {
            self.extract_standalone_type_specifiers(&decl.specifiers, offset);
            return;
        }

        for init_decl in &decl.declarators {
            let declarator = &init_decl.node.declarator;

            if is_typedef {
                self.extract_typedef(&decl.specifiers, &declarator.node, offset);
            } else if self.is_function_declarator(&declarator.node) {
                self.extract_function_declaration(
                    &decl.specifiers,
                    &declarator.node,
                    is_extern,
                    offset,
                );
            } else {
                self.extract_variable_or_type(
                    &decl.specifiers,
                    &declarator.node,
                    is_extern,
                    offset,
                );
            }
        }
    }

    fn extract_function_definition(&mut self, fdef: &FunctionDefinition, offset: usize) {
        // Function definitions have bodies — we extract signature only
        let name = self.declarator_name(&fdef.declarator.node);
        let Some(name) = name else { return };

        let base_ty = self.resolve_base_type(&fdef.specifiers);
        let (return_type, params, variadic) =
            self.resolve_function_parts(&fdef.declarator.node, base_ty);

        self.items.push(BindingItem::Function(FunctionBinding {
            name,
            calling_convention: CallingConvention::C,
            parameters: params,
            return_type,
            variadic,
            source_offset: Some(offset),
        }));
    }

    fn extract_standalone_type_specifiers(
        &mut self,
        specifiers: &[Node<DeclarationSpecifier>],
        offset: usize,
    ) {
        for spec in specifiers {
            match &spec.node {
                DeclarationSpecifier::TypeSpecifier(ts) => match &ts.node {
                    TypeSpecifier::Struct(st) => {
                        self.extract_record(&st.node, offset);
                    }
                    TypeSpecifier::Enum(et) => {
                        self.extract_enum(&et.node, offset);
                    }
                    _ => {}
                },
                _ => {}
            }
        }
    }

    fn extract_typedef(
        &mut self,
        specifiers: &[Node<DeclarationSpecifier>],
        declarator: &Declarator,
        offset: usize,
    ) {
        let name = match self.declarator_name(declarator) {
            Some(n) => n,
            None => return,
        };

        // Also extract any inline struct/enum definitions from the specifiers
        self.extract_inline_type_definitions(specifiers, offset);

        let target = self.resolve_full_type(specifiers, declarator);
        self.items.push(BindingItem::TypeAlias(TypeAliasBinding {
            name,
            target,
            source_offset: Some(offset),
        }));
    }

    fn extract_function_declaration(
        &mut self,
        specifiers: &[Node<DeclarationSpecifier>],
        declarator: &Declarator,
        _is_extern: bool,
        offset: usize,
    ) {
        let name = match self.declarator_name(declarator) {
            Some(n) => n,
            None => return,
        };

        let base_ty = self.resolve_base_type(specifiers);
        let (return_type, params, variadic) =
            self.resolve_function_parts(declarator, base_ty);

        self.items.push(BindingItem::Function(FunctionBinding {
            name,
            calling_convention: CallingConvention::C,
            parameters: params,
            return_type,
            variadic,
            source_offset: Some(offset),
        }));
    }

    fn extract_variable_or_type(
        &mut self,
        specifiers: &[Node<DeclarationSpecifier>],
        declarator: &Declarator,
        is_extern: bool,
        offset: usize,
    ) {
        // Extract any inline struct/enum definitions
        self.extract_inline_type_definitions(specifiers, offset);

        if is_extern {
            let name = match self.declarator_name(declarator) {
                Some(n) => n,
                None => return,
            };
            let ty = self.resolve_full_type(specifiers, declarator);
            self.items.push(BindingItem::Variable(VariableBinding {
                name,
                ty,
                source_offset: Some(offset),
            }));
        }
    }

    fn extract_inline_type_definitions(
        &mut self,
        specifiers: &[Node<DeclarationSpecifier>],
        offset: usize,
    ) {
        for spec in specifiers {
            if let DeclarationSpecifier::TypeSpecifier(ts) = &spec.node {
                match &ts.node {
                    TypeSpecifier::Struct(st) if st.node.declarations.is_some() => {
                        self.extract_record(&st.node, offset);
                    }
                    TypeSpecifier::Enum(et) if !et.node.enumerators.is_empty() => {
                        self.extract_enum(&et.node, offset);
                    }
                    _ => {}
                }
            }
        }
    }

    fn extract_record(&mut self, st: &StructType, offset: usize) {
        let kind = match st.kind.node {
            StructKind::Struct => RecordKind::Struct,
            StructKind::Union => RecordKind::Union,
        };
        let name = st.identifier.as_ref().map(|id| id.node.name.clone());

        let fields = st.declarations.as_ref().map(|decls| {
            let mut fields = Vec::new();
            for decl in decls {
                match &decl.node {
                    StructDeclaration::Field(field) => {
                        self.extract_struct_fields(&field.node, &mut fields);
                    }
                    StructDeclaration::StaticAssert(_) => {}
                }
            }
            fields
        });

        self.items.push(BindingItem::Record(RecordBinding {
            kind,
            name,
            fields,
            source_offset: Some(offset),
        }));
    }

    fn extract_struct_fields(&mut self, field: &StructField, out: &mut Vec<FieldBinding>) {
        let base_type_specs: Vec<_> = field
            .specifiers
            .iter()
            .filter_map(|sq| match &sq.node {
                SpecifierQualifier::TypeSpecifier(ts) => Some(ts),
                _ => None,
            })
            .collect();

        if field.declarators.is_empty() {
            // Anonymous field (e.g. anonymous struct/union)
            let ty = self.resolve_base_type_from_type_specs(&base_type_specs);
            out.push(FieldBinding { name: None, ty });
            return;
        }

        for sd in &field.declarators {
            let name = sd
                .node
                .declarator
                .as_ref()
                .and_then(|d| self.declarator_name(&d.node));
            let ty = match &sd.node.declarator {
                Some(d) => self.apply_derived_type(
                    self.resolve_base_type_from_type_specs(&base_type_specs),
                    &d.node,
                ),
                None => self.resolve_base_type_from_type_specs(&base_type_specs),
            };
            out.push(FieldBinding { name, ty });
        }
    }

    fn extract_enum(&mut self, et: &EnumType, offset: usize) {
        let name = et.identifier.as_ref().map(|id| id.node.name.clone());
        let variants: Vec<EnumVariant> = et
            .enumerators
            .iter()
            .map(|e| EnumVariant {
                name: e.node.identifier.node.name.clone(),
                value: e.node.expression.as_ref().and_then(|expr| eval_const_expr(&expr.node)),
            })
            .collect();

        self.items.push(BindingItem::Enum(EnumBinding {
            name,
            variants,
            source_offset: Some(offset),
        }));
    }

    // --- Type resolution helpers ---

    fn resolve_base_type(&self, specifiers: &[Node<DeclarationSpecifier>]) -> BindingType {
        let type_specs: Vec<_> = specifiers
            .iter()
            .filter_map(|s| match &s.node {
                DeclarationSpecifier::TypeSpecifier(ts) => Some(ts),
                _ => None,
            })
            .collect();
        self.resolve_base_type_from_type_specs(&type_specs)
    }

    fn resolve_base_type_from_type_specs(
        &self,
        type_specs: &[&Node<TypeSpecifier>],
    ) -> BindingType {
        if type_specs.is_empty() {
            return BindingType::Int; // C default
        }

        // Collect primitive specifier keywords
        let mut has_void = false;
        let mut has_char = false;
        let mut has_short = false;
        let mut has_int = false;
        let mut long_count = 0u8;
        let mut has_float = false;
        let mut has_double = false;
        let mut has_signed = false;
        let mut has_unsigned = false;
        let mut has_bool = false;

        for ts in type_specs {
            match &ts.node {
                TypeSpecifier::Void => has_void = true,
                TypeSpecifier::Char => has_char = true,
                TypeSpecifier::Short => has_short = true,
                TypeSpecifier::Int => has_int = true,
                TypeSpecifier::Long => long_count += 1,
                TypeSpecifier::Float => has_float = true,
                TypeSpecifier::Double => has_double = true,
                TypeSpecifier::Signed => has_signed = true,
                TypeSpecifier::Unsigned => has_unsigned = true,
                TypeSpecifier::Bool => has_bool = true,
                TypeSpecifier::Struct(st) => {
                    let name = st
                        .node
                        .identifier
                        .as_ref()
                        .map(|id| id.node.name.clone())
                        .unwrap_or_else(|| "<anonymous>".into());
                    return BindingType::RecordRef(name);
                }
                TypeSpecifier::Enum(et) => {
                    let name = et
                        .node
                        .identifier
                        .as_ref()
                        .map(|id| id.node.name.clone())
                        .unwrap_or_else(|| "<anonymous>".into());
                    return BindingType::EnumRef(name);
                }
                TypeSpecifier::TypedefName(id) => {
                    return BindingType::TypedefRef(id.node.name.clone());
                }
                _ => {}
            }
        }

        if has_void {
            return BindingType::Void;
        }
        if has_bool {
            return BindingType::Bool;
        }
        if has_float {
            return BindingType::Float;
        }
        if has_double && long_count > 0 {
            return BindingType::LongDouble;
        }
        if has_double {
            return BindingType::Double;
        }
        if has_char {
            return if has_unsigned {
                BindingType::UChar
            } else if has_signed {
                BindingType::SChar
            } else {
                BindingType::Char
            };
        }
        if has_short {
            return if has_unsigned {
                BindingType::UShort
            } else {
                BindingType::Short
            };
        }
        if long_count >= 2 {
            return if has_unsigned {
                BindingType::ULongLong
            } else {
                BindingType::LongLong
            };
        }
        if long_count == 1 {
            return if has_unsigned {
                BindingType::ULong
            } else {
                BindingType::Long
            };
        }
        if has_unsigned {
            return BindingType::UInt;
        }
        if has_signed || has_int {
            return BindingType::Int;
        }

        BindingType::Int
    }

    fn resolve_full_type(
        &self,
        specifiers: &[Node<DeclarationSpecifier>],
        declarator: &Declarator,
    ) -> BindingType {
        let base = self.resolve_base_type(specifiers);
        self.apply_derived_type(base, declarator)
    }

    fn apply_derived_type(&self, base: BindingType, declarator: &Declarator) -> BindingType {
        let mut ty = base;

        // Process derived declarators in order
        // In C declarator syntax, the derived list goes from innermost to outermost
        // Pointers are leftmost (first), arrays/functions are rightmost (last)
        // We need to split: pointers wrap inside-out, arrays/functions wrap outside-in
        let mut pointer_count = 0;
        for derived in &declarator.derived {
            match &derived.node {
                DerivedDeclarator::Pointer(_) => {
                    pointer_count += 1;
                }
                DerivedDeclarator::Array(arr) => {
                    let size = match &arr.node.size {
                        ArraySize::Unknown | ArraySize::VariableUnknown => None,
                        ArraySize::VariableExpression(expr)
                        | ArraySize::StaticExpression(expr) => {
                            eval_const_expr(&expr.node).map(|v| v as u64)
                        }
                    };
                    ty = BindingType::Array(Box::new(ty), size);
                }
                DerivedDeclarator::Function(fdecl) => {
                    let params = self.extract_parameters(&fdecl.node.parameters);
                    let variadic = fdecl.node.ellipsis == Ellipsis::Some;
                    ty = BindingType::FunctionPointer {
                        return_type: Box::new(ty),
                        parameters: params.iter().map(|p| p.ty.clone()).collect(),
                        variadic,
                    };
                }
                _ => {}
            }
        }

        for _ in 0..pointer_count {
            ty = BindingType::Pointer(Box::new(ty));
        }

        // Handle nested declarator (parenthesized)
        if let DeclaratorKind::Declarator(inner) = &declarator.kind.node {
            ty = self.apply_derived_type(ty, &inner.node);
        }

        ty
    }

    fn resolve_function_parts(
        &self,
        declarator: &Declarator,
        base_return_type: BindingType,
    ) -> (BindingType, Vec<ParameterBinding>, bool) {
        let mut return_type = base_return_type;
        let mut params = Vec::new();
        let mut variadic = false;

        // Apply pointer derivations to return type, find the function declarator
        for derived in &declarator.derived {
            match &derived.node {
                DerivedDeclarator::Pointer(_) => {
                    return_type = BindingType::Pointer(Box::new(return_type));
                }
                DerivedDeclarator::Function(fdecl) => {
                    params = self.extract_parameters(&fdecl.node.parameters);
                    variadic = fdecl.node.ellipsis == Ellipsis::Some;
                }
                _ => {}
            }
        }

        (return_type, params, variadic)
    }

    fn extract_parameters(
        &self,
        params: &[Node<ParameterDeclaration>],
    ) -> Vec<ParameterBinding> {
        // Handle `void` parameter (single unnamed void param means no params)
        if params.len() == 1 {
            let p = &params[0].node;
            if p.declarator.is_none() {
                let base = self.resolve_base_type_from_param_specifiers(&p.specifiers);
                if base == BindingType::Void {
                    return Vec::new();
                }
            }
        }

        params
            .iter()
            .map(|p| {
                let name = p
                    .node
                    .declarator
                    .as_ref()
                    .and_then(|d| self.declarator_name(&d.node));
                let base = self.resolve_base_type_from_param_specifiers(&p.node.specifiers);
                let ty = match &p.node.declarator {
                    Some(d) => self.apply_derived_type(base, &d.node),
                    None => base,
                };
                ParameterBinding { name, ty }
            })
            .collect()
    }

    fn resolve_base_type_from_param_specifiers(
        &self,
        specifiers: &[Node<DeclarationSpecifier>],
    ) -> BindingType {
        let type_specs: Vec<_> = specifiers
            .iter()
            .filter_map(|s| match &s.node {
                DeclarationSpecifier::TypeSpecifier(ts) => Some(ts),
                _ => None,
            })
            .collect();
        self.resolve_base_type_from_type_specs(&type_specs)
    }

    fn is_function_declarator(&self, declarator: &Declarator) -> bool {
        declarator.derived.iter().any(|d| {
            matches!(d.node, DerivedDeclarator::Function(_))
        })
    }

    fn declarator_name(&self, declarator: &Declarator) -> Option<String> {
        match &declarator.kind.node {
            DeclaratorKind::Identifier(id) => Some(id.node.name.clone()),
            DeclaratorKind::Declarator(inner) => self.declarator_name(&inner.node),
            DeclaratorKind::Abstract => None,
        }
    }
}

/// Best-effort constant expression evaluation for enum values and array sizes.
fn eval_const_expr(expr: &Expression) -> Option<i128> {
    match expr {
        Expression::Constant(c) => match &c.node {
            Constant::Integer(i) => {
                let s = i.number.as_ref();
                let val = match i.base {
                    IntegerBase::Decimal => i128::from_str_radix(s, 10).ok(),
                    IntegerBase::Octal => i128::from_str_radix(s, 8).ok(),
                    IntegerBase::Hexadecimal => i128::from_str_radix(s, 16).ok(),
                    IntegerBase::Binary => i128::from_str_radix(s, 2).ok(),
                };
                val
            }
            _ => None,
        },
        Expression::UnaryOperator(u) => {
            let inner = eval_const_expr(&u.node.operand.node)?;
            match u.node.operator.node {
                UnaryOperator::Minus => Some(-inner),
                UnaryOperator::Plus => Some(inner),
                UnaryOperator::Complement => Some(!inner),
                _ => None,
            }
        }
        Expression::BinaryOperator(b) => {
            let lhs = eval_const_expr(&b.node.lhs.node)?;
            let rhs = eval_const_expr(&b.node.rhs.node)?;
            match b.node.operator.node {
                BinaryOperator::Plus => Some(lhs + rhs),
                BinaryOperator::Minus => Some(lhs - rhs),
                BinaryOperator::Multiply => Some(lhs * rhs),
                BinaryOperator::Divide if rhs != 0 => Some(lhs / rhs),
                BinaryOperator::Modulo if rhs != 0 => Some(lhs % rhs),
                BinaryOperator::ShiftLeft => Some(lhs << (rhs as u32)),
                BinaryOperator::ShiftRight => Some(lhs >> (rhs as u32)),
                BinaryOperator::BitwiseAnd => Some(lhs & rhs),
                BinaryOperator::BitwiseOr => Some(lhs | rhs),
                BinaryOperator::BitwiseXor => Some(lhs ^ rhs),
                _ => None,
            }
        }
        _ => None,
    }
}

/// Parse a C source string and extract bindings.
pub fn extract_from_source(source: &str) -> Result<BindingPackage, String> {
    let unit = pac::parse::translation_unit(source, pac::driver::Flavor::GnuC11)
        .map_err(|e| format!("parse error at line {}:{}: {:?}", e.line, e.column, e.expected))?;

    let extractor = Extractor::new();
    let (items, diagnostics) = extractor.extract(&unit);

    Ok(BindingPackage {
        source_path: None,
        items,
        diagnostics,
    })
}

/// Extract bindings from an already-parsed translation unit.
pub fn extract_from_translation_unit(
    unit: &TranslationUnit,
    source_path: Option<String>,
) -> BindingPackage {
    let extractor = Extractor::new();
    let (items, diagnostics) = extractor.extract(unit);

    BindingPackage {
        source_path,
        items,
        diagnostics,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn extract(source: &str) -> BindingPackage {
        extract_from_source(source).expect("parse failed")
    }

    #[test]
    fn extract_typedef_int() {
        let pkg = extract("typedef int my_int;");
        assert_eq!(pkg.items.len(), 1);
        match &pkg.items[0] {
            BindingItem::TypeAlias(ta) => {
                assert_eq!(ta.name, "my_int");
                assert_eq!(ta.target, BindingType::Int);
            }
            other => panic!("expected TypeAlias, got {:?}", other),
        }
    }

    #[test]
    fn extract_typedef_pointer() {
        let pkg = extract("typedef void *handle_t;");
        assert_eq!(pkg.items.len(), 1);
        match &pkg.items[0] {
            BindingItem::TypeAlias(ta) => {
                assert_eq!(ta.name, "handle_t");
                assert_eq!(ta.target, BindingType::Pointer(Box::new(BindingType::Void)));
            }
            other => panic!("expected TypeAlias, got {:?}", other),
        }
    }

    #[test]
    fn extract_typedef_unsigned_long() {
        let pkg = extract("typedef unsigned long size_t;");
        match &pkg.items[0] {
            BindingItem::TypeAlias(ta) => {
                assert_eq!(ta.name, "size_t");
                assert_eq!(ta.target, BindingType::ULong);
            }
            other => panic!("expected TypeAlias, got {:?}", other),
        }
    }

    #[test]
    fn extract_extern_function() {
        let pkg = extract("extern int puts(const char *s);");
        assert_eq!(pkg.items.len(), 1);
        match &pkg.items[0] {
            BindingItem::Function(f) => {
                assert_eq!(f.name, "puts");
                assert_eq!(f.return_type, BindingType::Int);
                assert!(!f.variadic);
                assert_eq!(f.parameters.len(), 1);
                assert_eq!(f.parameters[0].name.as_deref(), Some("s"));
                assert_eq!(
                    f.parameters[0].ty,
                    BindingType::Pointer(Box::new(BindingType::Char))
                );
            }
            other => panic!("expected Function, got {:?}", other),
        }
    }

    #[test]
    fn extract_variadic_function() {
        let pkg = extract("int printf(const char *fmt, ...);");
        match &pkg.items[0] {
            BindingItem::Function(f) => {
                assert_eq!(f.name, "printf");
                assert!(f.variadic);
                assert_eq!(f.parameters.len(), 1);
            }
            other => panic!("expected Function, got {:?}", other),
        }
    }

    #[test]
    fn extract_void_function() {
        let pkg = extract("void abort(void);");
        match &pkg.items[0] {
            BindingItem::Function(f) => {
                assert_eq!(f.name, "abort");
                assert_eq!(f.return_type, BindingType::Void);
                assert!(f.parameters.is_empty());
            }
            other => panic!("expected Function, got {:?}", other),
        }
    }

    #[test]
    fn extract_enum() {
        let pkg = extract("enum color { RED, GREEN = 5, BLUE };");
        // Should have an Enum item
        let enums: Vec<_> = pkg.items.iter().filter_map(|i| match i {
            BindingItem::Enum(e) => Some(e),
            _ => None,
        }).collect();
        assert_eq!(enums.len(), 1);
        let e = &enums[0];
        assert_eq!(e.name.as_deref(), Some("color"));
        assert_eq!(e.variants.len(), 3);
        assert_eq!(e.variants[0].name, "RED");
        assert_eq!(e.variants[0].value, None);
        assert_eq!(e.variants[1].name, "GREEN");
        assert_eq!(e.variants[1].value, Some(5));
        assert_eq!(e.variants[2].name, "BLUE");
    }

    #[test]
    fn extract_struct_with_fields() {
        let pkg = extract("struct point { int x; int y; };");
        let records: Vec<_> = pkg.items.iter().filter_map(|i| match i {
            BindingItem::Record(r) => Some(r),
            _ => None,
        }).collect();
        assert_eq!(records.len(), 1);
        let r = &records[0];
        assert_eq!(r.kind, RecordKind::Struct);
        assert_eq!(r.name.as_deref(), Some("point"));
        assert!(!r.is_opaque());
        let fields = r.fields.as_ref().unwrap();
        assert_eq!(fields.len(), 2);
        assert_eq!(fields[0].name.as_deref(), Some("x"));
        assert_eq!(fields[0].ty, BindingType::Int);
    }

    #[test]
    fn extract_opaque_struct() {
        let pkg = extract("struct FILE;");
        let records: Vec<_> = pkg.items.iter().filter_map(|i| match i {
            BindingItem::Record(r) => Some(r),
            _ => None,
        }).collect();
        assert_eq!(records.len(), 1);
        assert!(records[0].is_opaque());
        assert_eq!(records[0].name.as_deref(), Some("FILE"));
    }

    #[test]
    fn extract_union() {
        let pkg = extract("union data { int i; float f; };");
        let records: Vec<_> = pkg.items.iter().filter_map(|i| match i {
            BindingItem::Record(r) => Some(r),
            _ => None,
        }).collect();
        assert_eq!(records.len(), 1);
        assert_eq!(records[0].kind, RecordKind::Union);
        assert_eq!(records[0].name.as_deref(), Some("data"));
    }

    #[test]
    fn extract_function_returning_pointer() {
        let pkg = extract("void *malloc(unsigned long size);");
        match &pkg.items[0] {
            BindingItem::Function(f) => {
                assert_eq!(f.name, "malloc");
                assert_eq!(
                    f.return_type,
                    BindingType::Pointer(Box::new(BindingType::Void))
                );
                assert_eq!(f.parameters.len(), 1);
                assert_eq!(f.parameters[0].ty, BindingType::ULong);
            }
            other => panic!("expected Function, got {:?}", other),
        }
    }

    #[test]
    fn extract_extern_variable() {
        let pkg = extract("extern int errno;");
        match &pkg.items[0] {
            BindingItem::Variable(v) => {
                assert_eq!(v.name, "errno");
                assert_eq!(v.ty, BindingType::Int);
            }
            other => panic!("expected Variable, got {:?}", other),
        }
    }

    #[test]
    fn extract_typedef_struct() {
        let pkg = extract("typedef struct point { int x; int y; } point_t;");
        // Should produce a Record and a TypeAlias
        let records: Vec<_> = pkg.items.iter().filter_map(|i| match i {
            BindingItem::Record(r) => Some(r),
            _ => None,
        }).collect();
        let aliases: Vec<_> = pkg.items.iter().filter_map(|i| match i {
            BindingItem::TypeAlias(ta) => Some(ta),
            _ => None,
        }).collect();
        assert_eq!(records.len(), 1);
        assert_eq!(aliases.len(), 1);
        assert_eq!(records[0].name.as_deref(), Some("point"));
        assert_eq!(aliases[0].name, "point_t");
        assert_eq!(aliases[0].target, BindingType::RecordRef("point".into()));
    }

    #[test]
    fn extract_multiple_declarations() {
        let src = r#"
            typedef unsigned long size_t;
            void *malloc(size_t size);
            void free(void *ptr);
            struct FILE;
            extern int errno;
        "#;
        let pkg = extract(src);
        assert_eq!(pkg.items.len(), 5);
    }

    #[test]
    fn extract_function_pointer_typedef() {
        let pkg = extract("typedef void (*handler_t)(int sig);");
        match &pkg.items[0] {
            BindingItem::TypeAlias(ta) => {
                assert_eq!(ta.name, "handler_t");
                match &ta.target {
                    BindingType::Pointer(inner) => match inner.as_ref() {
                        BindingType::FunctionPointer { return_type, parameters, variadic } => {
                            assert_eq!(**return_type, BindingType::Void);
                            assert_eq!(parameters.len(), 1);
                            assert_eq!(parameters[0], BindingType::Int);
                            assert!(!variadic);
                        }
                        other => panic!("expected FunctionPointer inside Pointer, got {:?}", other),
                    },
                    other => panic!("expected Pointer, got {:?}", other),
                }
            }
            other => panic!("expected TypeAlias, got {:?}", other),
        }
    }

    #[test]
    fn ir_determinism() {
        let src = r#"
            typedef int int32_t;
            enum status { OK = 0, ERR = 1 };
            struct point { int x; int y; };
            void *malloc(unsigned long size);
        "#;
        let pkg1 = extract(src);
        let pkg2 = extract(src);
        let json1 = serde_json::to_string(&pkg1).unwrap();
        let json2 = serde_json::to_string(&pkg2).unwrap();
        assert_eq!(json1, json2);
    }
}
