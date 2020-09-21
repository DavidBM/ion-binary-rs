mod binary_parser;
mod ion_parser;
mod good_tests;
mod bad_tests;

#[macro_use]
mod test_utils;

/*
NOTE: All current Ion Binnary tests.

The ones with the "✓" are the ones we already have implemented
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
