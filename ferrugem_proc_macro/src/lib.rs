rouille_compilogenese::rouille! {
    utilisons macro_procédurale::{Groupe, Identifiant, FluxDeJetons, ArbreDeJetons};

    fonction remplacer_identifiant(identifiant: Identifiant) -> PeutÊtre<ArbreDeJetons> {
        soit identifiant_chaine = identifiant.vers_chaine();

        soit nouvelle_chaine = selon identifiant_chaine.en_tant_que_chaine() {
            "Erro" => "Err",
            "Bom" => "Ok",
            "Texto" => "String",
            "Dicionário" => "HashMap",
            "Padrão" => "Default",
            "Errozaço" => "Error",
            "Opção" => "Option",
            "Algum" => "Some",
            "Nenhum" => "None",
            "Resultado" => "Result",
            "Eu" => "Self",
            "imprime" => "println",
            "sair" => "break",
            "assíncrono" => "async",
            "espera" => "await",
            "laço" => "loop",
            "se_mexe" => "move",
            "caixa" => "crate",
            "código_inacessível" => "unreachable_code",
            "como" => "as",
            "constante" => "const",
            "convention" => "trait", //TODO
            "perigoso" => "unsafe",
            "em" => "in",
            "de" => "from",
            "dinâmico" => "dyn",
            "descascar" => "unwrap",
            "padrão" => "default",
            "como_referência" => "as_ref",
            "es" => "io",
            "externo" => "extern",
            "falso" => "false",
            "função" => "fn",
            "foda" => "super",
            "inserir" => "insert",
            "pegar" => "get",
            "permitir" => "allow",
            "merda" | "porra" | "opa" => "panic",
            "módulo" => "mod",
            "mutável" => "mut",
            "novo" => "new",
            "onde" => "where",
            "por" => "for",
            "pegar_ou_inserir_com" => "get_or_insert_with",
            "principal" => "main",
            "público" => "pub",
            "que" => Rien?, //TODO
            "retorna" => "return",
            "implementa" => "impl",
            "referência" => "ref",
            "combina" => "match",
            "se" => "if",
            "senão" => "else",
            "eu" => "self",
            "seja" => "let",
            "estático" => "static",
            "estrutura" => "struct",
            "confia" => "expect",
            "enquanto" => "while",
            "usando" => "use",
            "dentro_de" => "into",
            "verdadeiro" => "true",
            "enumeração" => "enum",
            "Grupo" => "Group",
            "Identificador" => "Ident",
            "FluxDeJetons" => "TokenStream", //TODO from here on
            "ArbreDeJetons" => "TokenTree",
            "vers_chaine" => "to_string",
            "en_tant_que_chaine" => "as_str",
            "portée" => "span",
            "Tableau" => "Vec",
            "flux" => "stream",
            "pousser" => "push",
            "étendre" => "extend",
            "délimiteur" => "delimiter",
            "Ponctuation" => "Punct",
            "Litéral" => "Literal",
            "macro_procédurale" => "proc_macro",
            _ => &identifiant_chaine,
        };

        soit nouvel_identifiant = Identifiant::nouveau(nouvelle_chaine, identifiant.portée());
        Quelque(ArbreDeJetons::Identifiant(nouvel_identifiant))
    }

    fonction remplacer_arbre(jeton: ArbreDeJetons, sortie: &mutable Tableau<ArbreDeJetons>) {
        selon jeton {
            ArbreDeJetons::Groupe(groupe) => {
                soit mutable groupe_elements = Tableau::nouveau();
                remplacer_le_flux(groupe.flux(), &mutable groupe_elements);
                soit mutable nouveau_flux = FluxDeJetons::nouveau();
                nouveau_flux.étendre(groupe_elements);
                sortie.pousser(ArbreDeJetons::Groupe(Groupe::nouveau(groupe.délimiteur(), nouveau_flux)));
            }
            ArbreDeJetons::Identifiant(identifiant) => {
                si soit Quelque(identifiant) = remplacer_identifiant(identifiant) {
                    sortie.pousser(identifiant);
                }
            }
            ArbreDeJetons::Ponctuation(..) | ArbreDeJetons::Litéral(..) => {
                sortie.pousser(jeton);
            }
        }
    }

    fonction remplacer_le_flux(arbre_de_jetons: FluxDeJetons, sortie: &mutable Tableau<ArbreDeJetons>) {
        pour jeton de arbre_de_jetons {
            remplacer_arbre(jeton, sortie)
        }
    }

    #[macro_procédurale]
    public fonction rouille(élément: FluxDeJetons) -> FluxDeJetons {
        soit mutable retourné = Tableau::nouveau();
        remplacer_le_flux(élément, &mutable retourné);
        soit mutable sortie = FluxDeJetons::nouveau();
        sortie.étendre(retourné);
        sortie
    }
}
