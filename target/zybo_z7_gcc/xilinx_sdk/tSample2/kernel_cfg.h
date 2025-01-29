/* kernel_cfg.h */
#ifndef TOPPERS_KERNEL_CFG_H
#define TOPPERS_KERNEL_CFG_H

#define TNUM_TSKID	11
#define TSKID_tTask_rProcessor1Migratable_Task1	1
#define TSKID_tTask_rProcessor1Migratable_Task2	2
#define TSKID_tTask_rProcessor2Symmetric_Task2_1	3
#define TSKID_tTask_rProcessor2Symmetric_Task2_2	4
#define TSKID_tTask_rProcessor2Symmetric_Task3	5
#define TSKID_tTask_rProcessor1Migratable_Task2_3	6
#define TSKID_tTask_rProcessor1Migratable_MainTask	7
#define TSKID_tTask_rProcessor1Migratable_ExceptionTask	8
#define TSKID_tTask_rProcessor2Symmetric_MainTask2	9
#define TSKID_tTask_rProcessor2Symmetric_ExceptionTask2	10
#define TSKID_tTask_rProcessor1Migratable_LogTask_Task	11

#define TNUM_SEMID	2
#define SEMID_tSemaphore_rProcessor1Migratable_SerialPort1_ReceiveSemaphore	1
#define SEMID_tSemaphore_rProcessor1Migratable_SerialPort1_SendSemaphore	2

#define TNUM_FLGID	0

#define TNUM_DTQID	0

#define TNUM_PDQID	0

#define TNUM_MTXID	0

#define TNUM_SPNID	0

#define TNUM_MPFID	0

#define TNUM_CYCID	2
#define CYCID_tCyclicHandler_rProcessor2Symmetric_CyclicHandler2	1
#define CYCID_tCyclicHandler_rProcessor1Migratable_CyclicHandler	2

#define TNUM_ALMID	2
#define ALMID_tAlarmHandler_rProcessor2Symmetric_AlarmHandler2	1
#define ALMID_tAlarmHandler_rProcessor1Migratable_AlarmHandler	2

#define TNUM_ISRID	3
#define ISRID_tISR_rProcessor1Migratable_InterruptServiceRoutine	1
#define ISRID_tISR_rProcessor2Symmetric_InterruptServiceRoutine2	2
#define ISRID_tISR_rProcessor1Migratable_SIOPortTarget1_ISRInstance	3

#define TNUM_INIRTN	3

#define TNUM_TERRTN	4

#endif /* TOPPERS_KERNEL_CFG_H */
