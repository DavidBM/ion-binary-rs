mod binary_parser;
mod ion_parser;
mod ion_parser_good;
mod ion_parser_good_decimal;
mod ion_parser_good_equivs;
mod ion_parser_good_equivs_int;
mod ion_parser_good_int;
mod ion_parser_good_nop_padding;
mod ion_parser_good_null;
mod ion_parser_good_struct;
mod ion_parser_good_timestamp;
mod ion_parser_good_typecodes;

#[macro_use]
mod test_utils;

/*
NOTE: All current Ion Binnary tests.

The ones with the "✓" are the ones we already have implemented

✓	good/clobWithDel
✓	good/intBigSize256
✓	good/intBigSize14
✓	good/structLen13
✓	good/clobWithNonAsciiCharacter
✓	good/clobWithNullCharacter
✓	good/decimalNegativeOneDotZero
✓	good/decimalNegativeZeroDot
✓	good/decimalNegativeZeroDotZero
✓	good/decimalOneDotZero
✓	good/decimalZeroDot
✓	good/emptyThreeByteNopPad
✓	good/float32
✓	good/intBigSize1201
✓	good/intBigSize13
✓	good/intBigSize16
✓	good/intLongMaxValuePlusOne
✓	good/intLongMinValue
✓	good/item1
✓	good/nopPad16Bytes
✓	good/nopPadInsideEmptyStructNonZeroSymbolId
✓	good/nopPadInsideEmptyStructZeroSymbolId
✓	good/nopPadInsideStructWithNopPadThenValueNonZeroSymbolId
✓	good/nopPadInsideStructWithNopPadThenValueZeroSymbolId
✓	good/nopPadInsideStructWithValueThenNopPad
✓	good/nopPadOneByte
✓	good/valueBetweenNopPads
✓	good/valueFollowedByNopPad
✓	good/valuePrecededByNopPad
✓	good/null
✓	good/nullBlob
✓	good/nullBool
✓	good/nullClob
✓	good/nullDecimal
✓	good/nullFloat
✓	good/nullInt2
✓	good/nullInt3
✓	good/nullList
✓	good/nullSexp
✓	good/nullString
✓	good/nullStruct
✓	good/nullSymbol
✓	good/nullTimestamp
✓	good/structAnnotatedEmpty
✓	good/structAnnotatedOrdered
✓	good/structEmpty
✓	good/structLen14
✓	good/structLen15
✓	good/structOrdered
✓	good/structUnordered
✓	good/symbolExplicitZero
✓	good/symbolImplicitZero
✓	good/testfile28
✓	good/equivs/intsLargeNegative1
✓	good/equivs/intsLargeNegative2
✓	good/equivs/intsLargeNegative3
✓	good/equivs/intsLargePositive1
✓	good/equivs/intsLargePositive2
✓	good/equivs/intsLargePositive3
✓	good/equivs/nopPadEmptyStruct
✓	good/equivs/nopPadNonEmptyStruct
✓	good/equivs/paddedInts
✓	good/equivs/timestampFractions
✓	good/equivs/timestampSuperfluousOffset
✓	good/timestamp/timestamp2011
✓	good/timestamp/timestamp2011-02
✓   good/timestamp/timestamp2011-02-20
✓   good/timestamp/timestamp2011-02-20T19_30_59_100-08_00
✓   good/typecodes/T0
✓   good/typecodes/T1
✓   good/typecodes/T2
✓   good/typecodes/T3
✓   good/typecodes/T4
✓   good/typecodes/T5
✓   good/typecodes/T6-large
✓   good/typecodes/T6-small
✓   good/typecodes/T7-large
✓   good/typecodes/T7-small
✓   good/typecodes/T8
✓   good/typecodes/T9
✓   good/typecodes/T10
✓   good/typecodes/T11
✓   good/typecodes/T12
✓   good/typecodes/T13
✓   good/typecodes/T14
✓   good/typecodes/T15
    ---
    bad/annotationLengthTooLongContainer
    bad/annotationLengthTooLongScalar
    bad/annotationLengthTooShortContainer
    bad/annotationLengthTooShortScalar
    bad/annotationNested
    bad/annotationSymbolIDUnmapped
    bad/annotationWithNoValue
    bad/badMagic1015
    bad/badMagicE00100E0
    bad/blobLenTooLarge
    bad/boolWithInvalidLength_1
    bad/boolWithInvalidLength_2
    bad/clobLenTooLarge
    bad/decimalExpTooLarge
    bad/decimalLenCauses64BitOverflow
    bad/decimalLenTooLarge
    bad/emptyAnnotatedInt
    bad/fieldNameSymbolIDUnmapped
    bad/floatLenTooLarge
    bad/listWithValueLargerThanSize
    bad/localSymbolTableWithMultipleImportsFields
    bad/localSymbolTableWithMultipleSymbolsAndImportsFields
    bad/localSymbolTableWithMultipleSymbolsFields
    bad/minLongWithLenTooLarge
    bad/minLongWithLenTooSmall
    bad/negativeIntZero
    bad/negativeIntZeroLn
    bad/nopPadTooShort
    bad/nopPadWithAnnotations
    bad/stringLenTooLarge
    bad/stringWithLatinEncoding
    bad/structOrderedEmpty
    bad/symbolIDUnmapped
    bad/symbolLenTooLarge
    bad/timestamp/outOfRange/leapDayNonLeapYear_1
    bad/timestamp/outOfRange/leapDayNonLeapYear_2
    bad/timestamp/timestampFraction10d-1
    bad/timestamp/timestampFraction11d-1
    bad/timestamp/timestampFraction1d0
    bad/timestamp/timestampHourWithoutMinute
    bad/timestamp/timestampLenTooLarge
    bad/timestamp/timestampNegativeFraction
    bad/timestamp/timestampSept31
    bad/typecodes/type_14_length_1
    bad/typecodes/type_14_length_15
    bad/typecodes/type_14_length_2
    bad/typecodes/type_15_length_0
    bad/typecodes/type_15_length_1
    bad/typecodes/type_15_length_10
    bad/typecodes/type_15_length_11
    bad/typecodes/type_15_length_12
    bad/typecodes/type_15_length_13
    bad/typecodes/type_15_length_14
    bad/typecodes/type_15_length_15
    bad/typecodes/type_15_length_2
    bad/typecodes/type_15_length_3
    bad/typecodes/type_15_length_4
    bad/typecodes/type_15_length_5
    bad/typecodes/type_15_length_6
    bad/typecodes/type_15_length_7
    bad/typecodes/type_15_length_8
    bad/typecodes/type_15_length_9
    bad/typecodes/type_1_length_10
    bad/typecodes/type_1_length_11
    bad/typecodes/type_1_length_12
    bad/typecodes/type_1_length_13
    bad/typecodes/type_1_length_14
    bad/typecodes/type_1_length_2
    bad/typecodes/type_1_length_3
    bad/typecodes/type_1_length_4
    bad/typecodes/type_1_length_5
    bad/typecodes/type_1_length_6
    bad/typecodes/type_1_length_7
    bad/typecodes/type_1_length_8
    bad/typecodes/type_1_length_9
    bad/typecodes/type_3_length_0
    bad/typecodes/type_4_length_1
    bad/typecodes/type_4_length_10
    bad/typecodes/type_4_length_11
    bad/typecodes/type_4_length_12
    bad/typecodes/type_4_length_13
    bad/typecodes/type_4_length_14
    bad/typecodes/type_4_length_2
    bad/typecodes/type_4_length_3
    bad/typecodes/type_4_length_5
    bad/typecodes/type_4_length_6
    bad/typecodes/type_4_length_7
    bad/typecodes/type_4_length_9
    bad/typecodes/type_6_length_0
    bad/typecodes/type_6_length_1
*/
