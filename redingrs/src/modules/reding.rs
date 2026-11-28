pub mod redingWork {

    use super::read_config_json::Aristas;
    use super::read_config_json::Nodos;
    use crate::modules::math::Grafos::Grafo;
    use std::collections::HashMap;

    pub fn grafos_(arista: Option<Vec<Aristas>>, nodos: Option<Vec<Nodos>>) {
        let mut grafosv = Grafo::new();
        if let (Some(nodos), Some(arista)) = (nodos, arista) {
            // Grafo asigna IDs `usize` consecutivos, mientras el JSON usa
            // letras (`A`, `B`, `C`) como identificadores
            let ids: HashMap<char, usize> = nodos
                .iter()
                .enumerate()
                .map(|(indice, nodo)| (nodo.id, indice))
                .collect();

            for n in nodos {
                grafosv.agregar_nodo(n.nombre);
            }
            for a in arista {
                let origen = *ids
                    .get(&a.origen)
                    .unwrap_or_else(|| panic!("No existe el nodo de origen {}", a.origen));
                let destino = *ids
                    .get(&a.destino)
                    .unwrap_or_else(|| panic!("No existe el nodo de destino {}", a.destino));

                grafosv.agregar_arista(origen, destino, a.peso_m);
                grafosv.ordenar_aristas_por_costo();
                grafosv.mostrar();
            }
        } else {
            let a = grafosv.agregar_nodo("Casa A");
            let b = grafosv.agregar_nodo("Casa B");
            let c = grafosv.agregar_nodo("Casa C");

            grafosv.agregar_arista(a, b, 5);
            grafosv.agregar_arista(a, c, 3);
            grafosv.agregar_arista(b, c, 2);
            grafosv.ordenar_aristas_por_costo();
            grafosv.mostrar();
        }
    }
}

pub mod read_config_json {
    use serde::{Deserialize, Serialize};
    use serde_json::Result;
    use std::fs::File;
    use std::io::BufReader;

    #[warn(non_snake_case)]
    #[derive(Deserialize, Debug)]
    pub struct Data {
        nodos: Vec<Nodos>,
        aristas: Vec<Aristas>,
    }

    #[derive(Debug, Deserialize)]
    pub struct Aristas {
        pub origen: char,
        pub destino: char,
        pub peso_m: i64,
    }

    #[derive(Debug, Deserialize)]
    pub struct Nodos {
        pub id: char,
        pub nombre: String,
        pub x: u16,
        pub y: u16,
    }

    pub fn archivo_config()
    -> std::result::Result<(Vec<Aristas>, Vec<Nodos>), Box<dyn std::error::Error>> {
        let fileConfig =
            File::open("/home/eduardx_2/Documentos/Reding/redingrs/src/modules/escolar.json")?;
        let readerArchive = BufReader::new(fileConfig);
        let data_json: Data = serde_json::from_reader(readerArchive)?;
        let arista: Vec<Aristas> = data_json.aristas.into_iter().collect();
        let nodo: Vec<Nodos> = data_json.nodos.into_iter().collect();
        Ok((arista, nodo))
    }
}
