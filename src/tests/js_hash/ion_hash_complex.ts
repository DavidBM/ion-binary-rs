import * as ion from 'ion-js';
import * as ionHash from 'ion-hash-js';
import * as crypto from "crypto";

// This function will print a strict ready to copy past into rust
calculate_hash();

function calculate_hash() {
	let hashWriter = ionHash.makeHashWriter(
		ion.makeBinaryWriter(),
		ionHash.cryptoHasherProvider('sha256'));

	hashWriter.addAnnotation("Annot 1");
	hashWriter.addAnnotation("Annot 2");
	hashWriter.addAnnotation("Annot 3");

	hashWriter.stepIn(ion.IonTypes.STRUCT);

	hashWriter.writeFieldName("e"); hashWriter.writeInt(5);
	hashWriter.writeFieldName("a"); create_long_struct(hashWriter);
	hashWriter.writeFieldName("l"); hashWriter.writeInt(12);
	hashWriter.writeFieldName("b"); hashWriter.writeInt(2);
	hashWriter.writeFieldName("i"); hashWriter.writeInt(9);
	hashWriter.writeFieldName("n"); hashWriter.writeFloat32(123.12);

	hashWriter.stepOut();

	let digest = hashWriter.digest();
	console.log('digest: ' + toHexString(digest));
}

function create_list(hashWriter: ionHash.HashWriter) {
	hashWriter.stepIn(ion.IonTypes.LIST);

	hashWriter.writeInt(1);
	hashWriter.writeInt(2);
	hashWriter.writeInt(3);
	hashWriter.writeInt(-3);
	hashWriter.writeInt(-3354654);

	hashWriter.stepOut();
}

function create_qldb(hashWriter: ionHash.HashWriter) {
	hashWriter.stepIn(ion.IonTypes.STRUCT);

	hashWriter.writeFieldName("Model"); hashWriter.writeString("CLK 350");
	hashWriter.writeFieldName("Type"); hashWriter.writeString("Sedan");
	hashWriter.writeFieldName("Color"); hashWriter.writeString("White");
	hashWriter.writeFieldName("VIN"); hashWriter.writeString("1C4RJFAG0FC625797");
	hashWriter.writeFieldName("Make"); hashWriter.writeString("Mercedes");
	hashWriter.writeFieldName("Year"); hashWriter.writeInt(2019);

	hashWriter.stepOut();
}

function create_long_struct(hashWriter: ionHash.HashWriter) {
	hashWriter.stepIn(ion.IonTypes.STRUCT);

	hashWriter.writeFieldName("000021i");	hashWriter.writeInt(9);
	hashWriter.writeFieldName("012i");		hashWriter.writeInt(9);
	hashWriter.writeFieldName("01d");		hashWriter.writeInt(4);
	hashWriter.writeFieldName("01h");		hashWriter.writeInt(8);
	hashWriter.writeFieldName("11n");		hashWriter.writeFloat32(NaN);
	hashWriter.writeFieldName("12l");		hashWriter.writeInt(12);
	hashWriter.writeFieldName("1d");		hashWriter.writeInt(4);
	hashWriter.writeFieldName("21l");		hashWriter.writeInt(12);
	hashWriter.writeFieldName("2h");		create_list(hashWriter);
	hashWriter.writeFieldName("aaa");		hashWriter.writeInt(1);
	hashWriter.writeFieldName("aak");		hashWriter.writeInt(11);
	hashWriter.writeFieldName("ae");		hashWriter.writeInt(5);
	hashWriter.writeFieldName("b");			create_qldb(hashWriter);
	hashWriter.writeFieldName("bb");		hashWriter.writeInt(2);
	hashWriter.writeFieldName("cb");		hashWriter.writeInt(2);
	hashWriter.writeFieldName("c");			hashWriter.writeInt(3);
	hashWriter.writeFieldName("d");			hashWriter.writeNull(ion.IonTypes.CLOB);
	hashWriter.writeFieldName("9f");		hashWriter.writeInt(6);
	hashWriter.writeFieldName("09f");		hashWriter.writeDecimal(new ion.Decimal("92407156491786485918754613897564897561387954629341564305176435762934857629384756024751649587623498561204576329654.1239476129586128957624351682956187465187324618724691845696216935"));
	hashWriter.writeFieldName("g");			hashWriter.writeInt(7);
	hashWriter.writeFieldName("00h");		hashWriter.writeInt(8);
	hashWriter.writeFieldName("0h");		hashWriter.writeInt(8);
	hashWriter.writeFieldName("i");			hashWriter.writeInt(9);
	hashWriter.writeFieldName("j");			hashWriter.writeInt(10);
	hashWriter.writeFieldName("k");			hashWriter.writeNull(ion.IonTypes.FLOAT);
	hashWriter.writeFieldName("00001l");	hashWriter.writeInt(12);
	hashWriter.writeFieldName("00002l");	hashWriter.writeInt(12);
	hashWriter.writeFieldName("10000l");	hashWriter.writeInt(12);
	hashWriter.writeFieldName("l");			hashWriter.writeInt(12);
	hashWriter.writeFieldName("m");			hashWriter.writeInt(13);
	hashWriter.writeFieldName("n");			hashWriter.writeInt(14);

	hashWriter.stepOut();
}

function toHexString(byteArray) {
    let sb = '';
    byteArray.forEach(b => {
        if (sb != '') { sb += '\\x' }
        sb += ('0' + (b & 0xFF).toString(16)).slice(-2);
    });
    return '\\x' + sb;
}
