/// Modelo de datos y operaciones básicas sobre grafos.
pub mod grafos {
    use std::collections::HashMap;

    use super::union_find::UnionFind;
    /// Vértice identificado por un `id` único dentro del grafo.
    #[derive(Debug, Clone, PartialEq, Eq)]
    pub struct Nodo {
        pub id: usize,
        pub nombre_n: String,
    }

    // Conexión entre dos identificadores de nodos.
    #[derive(Debug, Clone, PartialEq, Eq)]
    pub struct Arista {
        pub origen: usize,
        pub destino: usize,
        // Costo entero de la conexión; puede ser negativo.
        pub peso: i64,
    }

    // Conjunto de nodos y aristas. Al construirlo, los identificadores de los nodos deben ser únicos y
    // los extremos de cada arista deben referirse a nodos existentes.
    #[derive(Debug, Clone, Default, PartialEq, Eq)]
    pub struct Grafo {
        pub nodos: Vec<Nodo>,
        pub aristas: Vec<Arista>,
    }

    impl Grafo {
        // Crea un grafo sin nodos ni aristas.
        pub fn new() -> Self {
            Self::default()
        }

        // Agrega un nodo y devuelve su identificador para crear aristas.
        // Acepta tanto &str como String.
        pub fn agregar_nodo(&mut self, nombre: impl Into<String>) -> usize {
            let id = match self.nodos.iter().map(|nodo| nodo.id).max() {
                Some(ultimo_id) => ultimo_id
                    .checked_add(1)
                    .expect("No hay identificadores disponibles para otro nodo"),
                None => 0,
            };

            self.nodos.push(Nodo {
                id,
                nombre_n: nombre.into(),
            });
            id
        }

        // Agrega una arista con el costo indicado entre dos nodos existentes.
        // Produce un pánico si alguno de los identificadores no existe.
        pub fn agregar_arista(&mut self, origen: usize, destino: usize, peso: i64) {
            assert!(
                self.nodos.iter().any(|nodo| nodo.id == origen),
                "El nodo de origen {origen} no existe"
            );
            assert!(
                self.nodos.iter().any(|nodo| nodo.id == destino),
                "El nodo de destino {destino} no existe"
            );

            self.aristas.push(Arista {
                origen,
                destino,
                peso,
            });
        }

        /// Imprime los nodos y las aristas en su orden actual.
        pub fn mostrar(&self) {
            println!("Nodos:");
            for nodo in &self.nodos {
                println!("  {}: {}", nodo.id, nodo.nombre_n);
            }

            println!("Aristas:");
            for arista in &self.aristas {
                println!(
                    "  Origen: {}, destino: {}, costo: {}",
                    arista.origen, arista.destino, arista.peso
                );
            }
        }

        /// Ordena las aristas de menor a mayor costo, modificando el grafo.
        /// Las aristas con el mismo peso conservan su orden relativo.
        pub fn ordenar_aristas_por_costo(&mut self) {
            self.aristas.sort_by_key(|arista| arista.peso);
        }

        /// Calcula un árbol de expansión mínima con el algoritmo de Kruskal.
        ///
        /// Las aristas se procesan de menor a mayor peso. Una arista se acepta
        /// solo cuando `union` confirma que sus extremos pertenecen a grupos
        /// distintos; de lo contrario se descarta porque cerraría un ciclo.
        /// El resultado contiene como máximo `nodos - 1` aristas. Si el grafo
        /// no es conexo, devuelve un bosque de expansión mínima.
        pub fn kruskal(&self) -> Vec<Arista> {
            if self.nodos.len() < 2 {
                return Vec::new();
            }

            // El grafo expone IDs, que podrían no ser consecutivos. Union-Find
            // trabaja con índices compactos, por eso se crea este mapeo.
            let indices: HashMap<usize, usize> = self
                .nodos
                .iter()
                .enumerate()
                .map(|(indice, nodo)| (nodo.id, indice))
                .collect();

            let mut aristas = self.aristas.clone();
            aristas.sort_by_key(|arista| arista.peso);

            let mut conjuntos = UnionFind::new(self.nodos.len());
            let objetivo = self.nodos.len() - 1;
            let mut aceptadas = Vec::with_capacity(objetivo);

            for arista in aristas {
                let origen = *indices
                    .get(&arista.origen)
                    .expect("La arista referencia un nodo de origen inexistente");
                let destino = *indices
                    .get(&arista.destino)
                    .expect("La arista referencia un nodo de destino inexistente");

                if conjuntos.union(origen, destino) {
                    aceptadas.push(arista);
                    if aceptadas.len() == objetivo {
                        break;
                    }
                }
            }

            aceptadas
        }
    }
}

// Mantiene compatible el nombre del módulo original.
pub use grafos as Grafos;

/// Conjuntos disjuntos para consultar y unir componentes independientes.
pub mod union_find {
    /// Union-Find con compresión de caminos y unión por rango.
    ///
    /// Los elementos se identifican mediante índices en `0..cantidad`.
    /// Para usar identificadores de un grafo, deben pertenecer a ese intervalo;
    /// si son dispersos, se pueden mapear a índices consecutivos.
    #[derive(Debug, Clone)]
    pub struct UnionFind {
        padres: Vec<usize>,
        // Cota de la altura del árbol; solo se consulta en sus raíces.
        rangos: Vec<usize>,
    }

    impl UnionFind {
        /// Crea un conjunto independiente para cada elemento.
        pub fn new(cantidad: usize) -> Self {
            Self {
                padres: (0..cantidad).collect(),
                rangos: vec![0; cantidad],
            }
        }

        /// Devuelve el representante del conjunto y comprime todo el camino
        /// recorrido para que cada elemento apunte directamente a la raíz.
        ///
        /// # Panics
        /// Produce un pánico si `elemento` está fuera del intervalo válido.
        pub fn find(&mut self, elemento: usize) -> usize {
            self.validar_elemento(elemento);

            let mut raiz = elemento;
            while self.padres[raiz] != raiz {
                raiz = self.padres[raiz];
            }

            let mut actual = elemento;
            while self.padres[actual] != actual {
                let siguiente = self.padres[actual];
                self.padres[actual] = raiz;
                actual = siguiente;
            }

            raiz
        }

        /// Une dos conjuntos, colocando la raíz de menor rango bajo la de mayor.
        /// Si los rangos coinciden, incrementa únicamente el de la nueva raíz.
        ///
        /// Devuelve `true` si unió conjuntos distintos o `false` si los elementos
        /// ya pertenecían al mismo conjunto.
        ///
        /// # Panics
        /// Produce un pánico si alguno de los elementos está fuera del intervalo
        /// válido. Valida ambos antes de modificar la estructura.
        pub fn union(&mut self, a: usize, b: usize) -> bool {
            self.validar_elemento(a);
            self.validar_elemento(b);

            let mut raiz_a = self.find(a);
            let mut raiz_b = self.find(b);

            if raiz_a == raiz_b {
                return false;
            }

            if self.rangos[raiz_a] < self.rangos[raiz_b] {
                std::mem::swap(&mut raiz_a, &mut raiz_b);
            }

            self.padres[raiz_b] = raiz_a;
            if self.rangos[raiz_a] == self.rangos[raiz_b] {
                self.rangos[raiz_a] += 1;
            }

            true
        }

        fn validar_elemento(&self, elemento: usize) {
            assert!(
                elemento < self.padres.len(),
                "El elemento {elemento} está fuera del intervalo 0..{}",
                self.padres.len()
            );
        }
    }

    #[cfg(test)]
    mod tests {
        use super::UnionFind;

        #[test]
        fn inicia_con_conjuntos_independientes() {
            let mut conjuntos = UnionFind::new(4);
            for elemento in 0..4 {
                assert_eq!(conjuntos.find(elemento), elemento);
            }
        }

        #[test]
        fn comprime_todo_el_camino_hasta_la_raiz() {
            let mut conjuntos = UnionFind::new(8);
            for (a, b) in [(0, 1), (2, 3), (4, 5), (6, 7), (0, 2), (4, 6), (0, 4)] {
                assert!(conjuntos.union(a, b));
            }
            // Antes de consultar: 7 -> 6 -> 4 -> 0.
            assert_eq!(conjuntos.padres[7], 6);
            assert_eq!(conjuntos.padres[6], 4);
            assert_eq!(conjuntos.padres[4], 0);

            assert_eq!(conjuntos.find(7), 0);

            for elemento in [7, 6, 4] {
                assert_eq!(conjuntos.padres[elemento], 0);
            }
            assert_eq!(conjuntos.rangos[0], 3);
        }

        #[test]
        fn une_por_rango_independientemente_del_orden_de_argumentos() {
            for invertir in [false, true] {
                let mut conjuntos = UnionFind::new(3);
                conjuntos.union(0, 1);
                let raiz = conjuntos.find(0);
                assert_eq!(conjuntos.rangos[raiz], 1);

                assert!(if invertir {
                    conjuntos.union(2, 0)
                } else {
                    conjuntos.union(0, 2)
                });

                assert_eq!(conjuntos.find(2), raiz);
                assert_eq!(conjuntos.rangos[raiz], 1);
            }
        }

        #[test]
        fn unir_elementos_ya_conectados_no_aumenta_el_rango() {
            let mut conjuntos = UnionFind::new(3);
            assert!(!conjuntos.union(0, 0));
            assert!(conjuntos.union(0, 1));
            assert!(conjuntos.union(1, 2));
            assert!(!conjuntos.union(2, 0));
            let raiz = conjuntos.find(0);
            assert_eq!(conjuntos.find(2), raiz);
            assert_eq!(conjuntos.rangos[raiz], 1);
        }

        #[test]
        fn permite_crear_una_estructura_vacia() {
            let conjuntos = UnionFind::new(0);
            assert!(conjuntos.padres.is_empty());
            assert!(conjuntos.rangos.is_empty());
        }

        #[test]
        #[should_panic(expected = "El elemento 0 está fuera del intervalo 0..0")]
        fn find_rechaza_elementos_en_una_estructura_vacia() {
            UnionFind::new(0).find(0);
        }

        #[test]
        #[should_panic(expected = "El elemento 2 está fuera del intervalo 0..2")]
        fn union_rechaza_un_elemento_fuera_del_intervalo() {
            UnionFind::new(2).union(0, 2);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::grafos::{Arista, Grafo, Nodo};

    #[test]
    fn construye_y_ordena_el_grafo_de_casas() {
        let mut grafo = Grafo::new();
        let a = grafo.agregar_nodo("Casa A");
        let b = grafo.agregar_nodo("Casa B");
        let c = grafo.agregar_nodo("Casa C");
        let d = grafo.agregar_nodo(String::from("Casa D"));

        grafo.agregar_arista(a, b, 5);
        grafo.agregar_arista(a, c, 3);
        grafo.agregar_arista(b, d, 2);
        grafo.agregar_arista(c, d, 4);
        grafo.agregar_arista(a, d, 7);
        grafo.mostrar();
        grafo.ordenar_aristas_por_costo();

        assert_eq!([a, b, c, d], [0, 1, 2, 3]);
        assert_eq!(grafo.nodos[d].nombre_n, "Casa D");
        assert_eq!(
            grafo
                .aristas
                .iter()
                .map(|arista| (arista.origen, arista.destino, arista.peso))
                .collect::<Vec<_>>(),
            vec![(b, d, 2), (a, c, 3), (c, d, 4), (a, b, 5), (a, d, 7)]
        );
    }

    #[test]
    #[should_panic(expected = "El nodo de origen 1 no existe")]
    fn rechaza_aristas_con_origen_inexistente() {
        let mut grafo = Grafo::new();
        let a = grafo.agregar_nodo("Casa A");
        grafo.agregar_arista(1, a, 5);
    }

    #[test]
    #[should_panic(expected = "El nodo de destino 1 no existe")]
    fn rechaza_aristas_con_destino_inexistente() {
        let mut grafo = Grafo::new();
        let a = grafo.agregar_nodo("Casa A");
        grafo.agregar_arista(a, 1, 5);
    }

    #[test]
    fn ordena_costos_negativos_y_positivos_conservando_empates() {
        let mut grafo = Grafo {
            nodos: (0..3)
                .map(|id| Nodo {
                    id,
                    nombre_n: format!("Nodo {id}"),
                })
                .collect(),
            aristas: vec![
                Arista {
                    origen: 0,
                    destino: 1,
                    peso: 8,
                },
                Arista {
                    origen: 1,
                    destino: 2,
                    peso: -3,
                },
                Arista {
                    origen: 0,
                    destino: 2,
                    peso: 8,
                },
                Arista {
                    origen: 2,
                    destino: 0,
                    peso: 0,
                },
            ],
        };
        let nodos_originales = grafo.nodos.clone();
        let aristas_originales = grafo.aristas.clone();

        grafo.ordenar_aristas_por_costo();

        assert_eq!(grafo.nodos, nodos_originales);
        assert_eq!(
            grafo.aristas,
            vec![
                aristas_originales[1].clone(),
                aristas_originales[3].clone(),
                aristas_originales[0].clone(),
                aristas_originales[2].clone(),
            ]
        );
    }

    #[test]
    fn permite_ordenar_un_grafo_vacio() {
        let mut grafo = Grafo::default();

        grafo.ordenar_aristas_por_costo();

        assert!(grafo.aristas.is_empty());
        assert!(grafo.nodos.is_empty());
    }

    #[test]
    fn kruskal_acepta_las_aristas_baratas_y_descarta_ciclos() {
        let mut grafo = Grafo::new();
        let a = grafo.agregar_nodo("Casa A");
        let b = grafo.agregar_nodo("Casa B");
        let c = grafo.agregar_nodo("Casa C");
        let d = grafo.agregar_nodo("Casa D");

        grafo.agregar_arista(a, b, 5);
        grafo.agregar_arista(a, c, 3);
        grafo.agregar_arista(b, d, 2);
        grafo.agregar_arista(c, d, 4);
        grafo.agregar_arista(a, d, 7);

        let arbol = grafo.kruskal();

        assert_eq!(arbol.len(), grafo.nodos.len() - 1);
        assert_eq!(
            arbol.iter().map(|arista| arista.peso).collect::<Vec<_>>(),
            vec![2, 3, 4]
        );
    }

    #[test]
    fn kruskal_admite_ids_no_consecutivos_y_grafos_desconectados() {
        let grafo = Grafo {
            nodos: vec![
                Nodo {
                    id: 10,
                    nombre_n: "A".into(),
                },
                Nodo {
                    id: 20,
                    nombre_n: "B".into(),
                },
                Nodo {
                    id: 40,
                    nombre_n: "C".into(),
                },
            ],
            aristas: vec![Arista {
                origen: 10,
                destino: 20,
                peso: 1,
            }],
        };

        let bosque = grafo.kruskal();

        assert_eq!(bosque.len(), 1);
        assert_eq!((bosque[0].origen, bosque[0].destino), (10, 20));
    }
}
