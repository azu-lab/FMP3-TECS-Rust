/*  1 "./gen/tmp_C_src.c" */
/*  1 "<built-in>" */
/*  1 "<command-line>" */
/*  1 "./gen/tmp_C_src.c" */
/*  36 "./gen/tmp_C_src.c" */
typedef struct { int dummy; } va_list;


/*  1 "../../../../tecsgen/tecs/rpc/tecs_rpc.h" 1 */
/*  42 "../../../../tecsgen/tecs/rpc/tecs_rpc.h" */
/*  1 "../../../../tecsgen/tecs/tecs.h" 1 */
/*  42 "../../../../tecsgen/tecs/tecs.h" */
/*  1 "../../../../include/t_stddef.h" 1 */
/*  65 "../../../../include/t_stddef.h" */
/*  1 "../../../../target/zybo_z7_gcc/target_stddef.h" 1 */
/*  22 "../../../../target/zybo_z7_gcc/target_stddef.h" */
/*  1 "../../../../arch/arm_gcc/zynq7000/chip_stddef.h" 1 */
/*  23 "../../../../arch/arm_gcc/zynq7000/chip_stddef.h" */
/*  1 "/usr/lib/gcc/arm-none-eabi/10.3.1/include/stdint.h" 1 3 4 */
/*  34 "/usr/lib/gcc/arm-none-eabi/10.3.1/include/stdint.h" 3 4 */

/*  34 "/usr/lib/gcc/arm-none-eabi/10.3.1/include/stdint.h" 3 4 */
typedef signed char int8_t;


typedef short int int16_t;


typedef long int int32_t;


typedef long long int int64_t;


typedef unsigned char uint8_t;


typedef short unsigned int uint16_t;


typedef long unsigned int uint32_t;


typedef long long unsigned int uint64_t;




typedef signed char int_least8_t;
typedef short int int_least16_t;
typedef long int int_least32_t;
typedef long long int int_least64_t;
typedef unsigned char uint_least8_t;
typedef short unsigned int uint_least16_t;
typedef long unsigned int uint_least32_t;
typedef long long unsigned int uint_least64_t;



typedef int int_fast8_t;
typedef int int_fast16_t;
typedef int int_fast32_t;
typedef long long int int_fast64_t;
typedef unsigned int uint_fast8_t;
typedef unsigned int uint_fast16_t;
typedef unsigned int uint_fast32_t;
typedef long long unsigned int uint_fast64_t;




typedef int intptr_t;


typedef unsigned int uintptr_t;




typedef long long int intmax_t;
typedef long long unsigned int uintmax_t;
/*  24 "../../../../arch/arm_gcc/zynq7000/chip_stddef.h" 2 */



/*  1 "../../../../arch/gcc/tool_stddef.h" 1 */
/*  81 "../../../../arch/gcc/tool_stddef.h" */
/*  1 "/usr/lib/gcc/arm-none-eabi/10.3.1/include/stddef.h" 1 3 4 */
/*  143 "/usr/lib/gcc/arm-none-eabi/10.3.1/include/stddef.h" 3 4 */
typedef int ptrdiff_t;
/*  209 "/usr/lib/gcc/arm-none-eabi/10.3.1/include/stddef.h" 3 4 */
typedef unsigned int size_t;
/*  321 "/usr/lib/gcc/arm-none-eabi/10.3.1/include/stddef.h" 3 4 */
typedef unsigned int wchar_t;
/*  415 "/usr/lib/gcc/arm-none-eabi/10.3.1/include/stddef.h" 3 4 */
typedef struct {
  long long __max_align_ll ;
  long double __max_align_ld ;
/*  426 "/usr/lib/gcc/arm-none-eabi/10.3.1/include/stddef.h" 3 4 */
} max_align_t;
/*  82 "../../../../arch/gcc/tool_stddef.h" 2 */
/*  1 "/usr/lib/gcc/arm-none-eabi/10.3.1/include-fixed/limits.h" 1 3 4 */
/*  83 "../../../../arch/gcc/tool_stddef.h" 2 */
/*  193 "../../../../arch/gcc/tool_stddef.h" */

/*  193 "../../../../arch/gcc/tool_stddef.h" */
typedef float float32_t;
typedef double double64_t;
/*  28 "../../../../arch/arm_gcc/zynq7000/chip_stddef.h" 2 */




/*  1 "../../../../arch/arm_gcc/common/core_stddef.h" 1 */
/*  33 "../../../../arch/arm_gcc/zynq7000/chip_stddef.h" 2 */
/*  23 "../../../../target/zybo_z7_gcc/target_stddef.h" 2 */






static inline void
TOPPERS_assert_abort(void)
{
/*  46 "../../../../target/zybo_z7_gcc/target_stddef.h" */
 while (1) ;
}
/*  66 "../../../../include/t_stddef.h" 2 */
/*  84 "../../../../include/t_stddef.h" */
struct TOPPERS_dummy_t { int TOPPERS_dummy_field; };
typedef void (*TOPPERS_fp_t)(struct TOPPERS_dummy_t);






typedef int bool_t;

typedef signed int int_t;
typedef unsigned int uint_t;

typedef signed long long_t;
typedef unsigned long ulong_t;

typedef int_t FN;
typedef int_t ER;
typedef int_t ID;
typedef uint_t ATR;
typedef uint_t STAT;
typedef uint_t MODE;
typedef int_t PRI;
typedef uint32_t TMO;
typedef intptr_t EXINF;
typedef uint32_t RELTIM;

typedef uint64_t SYSTIM;



typedef uint32_t PRCTIM;

typedef uint32_t HRTCNT;







typedef TOPPERS_fp_t FP;

typedef int_t ER_BOOL;
typedef int_t ER_ID;
typedef int_t ER_UINT;

typedef uintptr_t MB_T;

typedef uint32_t ACPTN;
typedef struct acvct {
 ACPTN acptn1;
 ACPTN acptn2;
 ACPTN acptn3;
 ACPTN acptn4;
} ACVCT;
/*  43 "../../../../tecsgen/tecs/tecs.h" 2 */



typedef struct tag_int128_t { int64_t hi; int64_t lo; } int128_t;
typedef struct tag_uint128_t { int64_t hi; int64_t lo; } uint128_t;




typedef short short_t;
typedef unsigned short ushort_t;




typedef char char_t;


typedef signed char schar_t;
typedef unsigned char uchar_t;
/*  43 "../../../../tecsgen/tecs/rpc/tecs_rpc.h" 2 */
/*  40 "./gen/tmp_C_src.c" 2 */
