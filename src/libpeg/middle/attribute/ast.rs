// Copyright 2014 Pierre Talbot (IRCAM)

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at

//     http://www.apache.org/licenses/LICENSE-2.0

// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

pub use front::ast::{Expression_, Expression, CharacterInterval, CharacterClassExpr};
pub use front::ast::Expression_::*;

pub use rust::{ExtCtxt, Span, Spanned, SpannedIdent};
pub use middle::attribute::attribute::*;
pub use identifier::*;
pub use middle::semantics::ast::Grammar as SGrammar;
pub use middle::semantics::ast::Rule as SRule;

use attribute::model_checker;
use attribute::model::*;
use monad::partial::Partial;

pub use std::collections::HashMap;
pub use std::iter::FromIterator;

pub struct Grammar{
  pub name: Ident,
  pub rules: HashMap<Ident, Rule>,
  pub attributes: GrammarAttributes
}

impl Grammar
{
  pub fn new(cx: &ExtCtxt, sgrammar: SGrammar) -> Partial<Grammar>
  {
    let grammar_model = GrammarAttributes::model();
    let grammar_model = model_checker::check_all(cx, grammar_model, sgrammar.attributes);

    let rules_models: Vec<(Ident, AttributeArray)> =
       sgrammar.rules.iter()
      .map(|(id, r)| (id.clone(), r.attributes.clone()))
      .map(|(id, a)| (id, Grammar::make_rule_model(cx, a)))
      .collect();

    let rules = FromIterator::from_iter(
      sgrammar.rules.into_iter()
      .zip(rules_models.clone().into_iter())
      .map(|((id, rule), (_, m))| (id, rule, m))
      .map(|(id, rule, model)| (id, Rule::new(cx, rule, &model)))
    );

    let attributes = GrammarAttributes::new(cx, rules_models, grammar_model);

    let grammar = Grammar{
      name: sgrammar.name,
      rules: rules,
      attributes: attributes
    };
    Partial::Value(grammar)
  }

  fn make_rule_model(cx: &ExtCtxt, attrs: Vec<Attribute>) -> AttributeArray {
    let rule_model = RuleAttributes::model();
    model_checker::check_all(cx, rule_model, attrs)
  }
}

pub struct Rule{
  pub name: SpannedIdent,
  pub attributes: RuleAttributes,
  pub def: Box<Expression>,
}

impl Rule
{
  fn new(cx: &ExtCtxt, rule: SRule, attrs: &AttributeArray) -> Rule
  {
    let attributes = RuleAttributes::new(cx, attrs);
    Rule{
      name: rule.name,
      attributes: attributes,
      def: rule.def
    }
  }
}