use source_map::Span;

use crate::{ASTNode, TSXToken, TypeDeclaration, TypeId, TypeReference};

/// e.g. `type NumberArray = Array<number>`
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "self-rust-tokenize", derive(self_rust_tokenize::SelfRustTokenize))]
pub struct TypeAlias {
	pub type_name: TypeDeclaration,
	pub type_expression: TypeReference,
	pub type_id: TypeId,
	position: Span,
}

impl ASTNode for TypeAlias {
	fn get_position(&self) -> std::borrow::Cow<Span> {
		std::borrow::Cow::Borrowed(&self.position)
	}

	fn from_reader(
		reader: &mut impl tokenizer_lib::TokenReader<TSXToken, Span>,
		state: &mut crate::ParsingState,
		settings: &crate::ParseSettings,
	) -> crate::ParseResult<Self> {
		let start = reader.expect_next(TSXToken::Keyword(crate::TSXKeyword::Type))?;
		let type_name = TypeDeclaration::from_reader(reader, state, settings)?;
		reader.expect_next(TSXToken::Assign)?;
		let type_expression = TypeReference::from_reader(reader, state, settings)?;
		let position = start.union(&type_expression.get_position());
		Ok(Self { type_name, type_expression, type_id: TypeId::new(), position })
	}

	fn to_string_from_buffer<T: source_map::ToString>(
		&self,
		buf: &mut T,
		settings: &crate::ToStringSettingsAndData,
		depth: u8,
	) {
		if settings.0.include_types {
			buf.push_str("type ");
			self.type_name.to_string_from_buffer(buf, settings, depth);
			buf.push_str(" = ");
			self.type_expression.to_string_from_buffer(buf, settings, depth);
		}
	}
}
