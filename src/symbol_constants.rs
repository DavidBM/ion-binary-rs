#[derive(Debug)]
pub enum SystemSymbolTableType {
	Ion,
	Ion1_0,
	IonSymbolTable,
	Name,
	Version,
	Imports,
	Symbols,
	MaxId,
	IonSharedSymbolTable,
}