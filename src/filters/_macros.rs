macro_rules! define_filters {
    (
        $(
            $kind:ident => $type:ty
        ),* $(,)?
    ) => {
        #[derive(Copy, Clone, PartialEq, Eq, Debug)]
        pub enum FilterKindType {
            $( $kind ),*
        }

        pub enum FilterKind {
            $( $kind($type) ),*
        }

        impl FilterKind {
            pub fn from_path(kind: FilterKindType, path: &Path, config: &dyn FilterConfig) -> Self {
                match kind {
                    $( FilterKindType::$kind => FilterKind::$kind(<$type>::new_from_file(path, config)) ),*
                }
            }

            fn as_filter(&self) -> &dyn Filter {
                match self {
                    $( FilterKind::$kind(inner) => inner ),*
                }
            }
        }

        impl Filter for FilterKind {
            fn apply(&self, path: &Path) -> bool {
                self.as_filter().apply(path)
            }
        }
    };
}
