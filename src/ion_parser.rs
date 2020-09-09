use std::collections::HashMap;
use std::io::Read;
use crate::binary_parser::IonBinaryParser;
use crate::binary_parser_types::*;

#[derive(Debug)]
pub struct IonParser {
	parser: IonBinaryParser,
	system_symbol_table: HashMap<usize, SystemSymbolTableType>,
}

impl IonParser {
    pub fn new(reader: Box<dyn Read>) -> IonParser {
    	let mut system_symbol_table = HashMap::new();

    	system_symbol_table.insert(1, SystemSymbolTableType::Ion); 
    	system_symbol_table.insert(2, SystemSymbolTableType::Ion1_0); 
    	system_symbol_table.insert(3, SystemSymbolTableType::IonSymbolTable); 
    	system_symbol_table.insert(4, SystemSymbolTableType::Name); 
    	system_symbol_table.insert(5, SystemSymbolTableType::Version); 
    	system_symbol_table.insert(6, SystemSymbolTableType::Imports); 
    	system_symbol_table.insert(7, SystemSymbolTableType::Symbols); 
    	system_symbol_table.insert(8, SystemSymbolTableType::MaxId); 
    	system_symbol_table.insert(9, SystemSymbolTableType::IonSharedSymbolTable); 

        IonParser { 
        	parser: IonBinaryParser::new(reader),
        	system_symbol_table
        }
    }
}

/*
ION SYSTEM SYMBOL TABLE

Symbol ID   Symbol Name
1           $ion
2           $ion_1_0
3           $ion_symbol_table
4           name
5           version
6           imports
7           symbols
8           max_id
9           $ion_shared_symbol_table

Basically, for QLDB, the first value that the DB sends is:
Annotation: 
    notation: 3 ($ion_symbol_table which means that is a local symbol table)
    Struct
        Symbols (via the id 7)
        List: Which contains the list of new Symbols

... After consuming the annotation header
Annotation Length: Ok(38)
Annotation annot_length: Ok(1)
Annotation annot: Ok(3)
Annotation value header: Ok(ValueHeader { type: Struct, length: LongLength })
Annotation value length: Ok(34)
Annotation value first key: Ok(7)
Annotation value first value header: Ok(ValueHeader { type: List, length: LongLength })

In the list, symbols are added in consecutive IDs following their insert order. 
A symbol cannot replace an already existing symbol. So, the system symbols come first, 
later the imported symbols, and finally, the local symbols. 
 */
