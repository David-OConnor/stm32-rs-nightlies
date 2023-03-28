#[doc = "Register `SECSR` reader"]
pub struct R(crate::R<SECSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SECSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SECSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SECSR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `BSY` reader - busy flag BSY flag indicates that a FLASH memory is busy (write, erase, option byte change, OBK operations). It is set at the beginning of a flash memory operation and cleared when the operation finishes or an error occurs."]
pub type BSY_R = crate::BitReader<bool>;
#[doc = "Field `WBNE` reader - write buffer not empty flag WBNE flag is set when the flash interface is waiting for new data to complete the write buffer. In this state, the write buffer is not empty. WBNE is reset by hardware each time the write buffer is complete or the write buffer is emptied following one of the event below: the application software forces the write operation using FW bit in FLASH_SECCR the flash interface detects an error that involves data loss This bit cannot be reset by writing 0 directly by software. To reset it, clear the write buffer by performing any of the above listed actions, or send the missing data."]
pub type WBNE_R = crate::BitReader<bool>;
#[doc = "Field `DBNE` reader - data buffer not empty flag DBNE flag is set when the embedded flash memory interface is processing 6-bits ECC data in dedicated buffer. This bit cannot be set to 0 by software. The hardware resets it once the buffer is free."]
pub type DBNE_R = crate::BitReader<bool>;
#[doc = "Field `EOP` reader - end of operation flag EOP flag is set when a operation (program/erase) completes. An interrupt is generated if the EOPIE is set to. It is not necessary to reset EOP before starting a new operation. EOP bit is cleared by writing 1 to CLR_EOP bit in FLASH_SECCCR register."]
pub type EOP_R = crate::BitReader<bool>;
#[doc = "Field `WRPERR` reader - write protection error flag WRPERR flag is raised when a protection error occurs during a program operation. An interrupt is also generated if the WRPERRIE is set to 1. Writing 1 to CLR_WRPERR bit in FLASH_SECCCR register clears WRPERR."]
pub type WRPERR_R = crate::BitReader<bool>;
#[doc = "Field `PGSERR` reader - programming sequence error flag PGSERR flag is raised when a sequence error occurs. An interrupt is generated if the PGSERRIE bit is set to 1. Writing 1 to CLR_PGSERR bit in FLASH_SECCCR register clears PGSERR."]
pub type PGSERR_R = crate::BitReader<bool>;
#[doc = "Field `STRBERR` reader - strobe error flag STRBERR flag is raised when a strobe error occurs (when the master attempts to write several times the same byte in the write buffer). An interrupt is generated if the STRBERRIE bit is set to 1. Writing 1 to CLR_STRBERR bit in FLASH_SECCCR register clears STRBERR."]
pub type STRBERR_R = crate::BitReader<bool>;
#[doc = "Field `INCERR` reader - inconsistency error flag INCERR flag is raised when a inconsistency error occurs. An interrupt is generated if INCERRIE is set to 1. Writing 1 to CLR_INCERR bit in the FLASH_SECCCR register clears INCERR."]
pub type INCERR_R = crate::BitReader<bool>;
#[doc = "Field `OBKERR` reader - OBK general error flag OBKERR flag is raised when the OBK-HDPL signal from the SBS does not match the HDPL value associated with the key slot during access to the key location. Alternatively also when the ALT_SECT is unexpectedly changed while the write buffer is being filled."]
pub type OBKERR_R = crate::BitReader<bool>;
#[doc = "Field `OBKWERR` reader - OBK write error flag OBKWERR flag is raised when the address is not virgin on a write access to the OBK storage. Alternatively also when the OBK selector in the alternate sector is not virgin during a swap operation."]
pub type OBKWERR_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0 - busy flag BSY flag indicates that a FLASH memory is busy (write, erase, option byte change, OBK operations). It is set at the beginning of a flash memory operation and cleared when the operation finishes or an error occurs."]
    #[inline(always)]
    pub fn bsy(&self) -> BSY_R {
        BSY_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - write buffer not empty flag WBNE flag is set when the flash interface is waiting for new data to complete the write buffer. In this state, the write buffer is not empty. WBNE is reset by hardware each time the write buffer is complete or the write buffer is emptied following one of the event below: the application software forces the write operation using FW bit in FLASH_SECCR the flash interface detects an error that involves data loss This bit cannot be reset by writing 0 directly by software. To reset it, clear the write buffer by performing any of the above listed actions, or send the missing data."]
    #[inline(always)]
    pub fn wbne(&self) -> WBNE_R {
        WBNE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 3 - data buffer not empty flag DBNE flag is set when the embedded flash memory interface is processing 6-bits ECC data in dedicated buffer. This bit cannot be set to 0 by software. The hardware resets it once the buffer is free."]
    #[inline(always)]
    pub fn dbne(&self) -> DBNE_R {
        DBNE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 16 - end of operation flag EOP flag is set when a operation (program/erase) completes. An interrupt is generated if the EOPIE is set to. It is not necessary to reset EOP before starting a new operation. EOP bit is cleared by writing 1 to CLR_EOP bit in FLASH_SECCCR register."]
    #[inline(always)]
    pub fn eop(&self) -> EOP_R {
        EOP_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - write protection error flag WRPERR flag is raised when a protection error occurs during a program operation. An interrupt is also generated if the WRPERRIE is set to 1. Writing 1 to CLR_WRPERR bit in FLASH_SECCCR register clears WRPERR."]
    #[inline(always)]
    pub fn wrperr(&self) -> WRPERR_R {
        WRPERR_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - programming sequence error flag PGSERR flag is raised when a sequence error occurs. An interrupt is generated if the PGSERRIE bit is set to 1. Writing 1 to CLR_PGSERR bit in FLASH_SECCCR register clears PGSERR."]
    #[inline(always)]
    pub fn pgserr(&self) -> PGSERR_R {
        PGSERR_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - strobe error flag STRBERR flag is raised when a strobe error occurs (when the master attempts to write several times the same byte in the write buffer). An interrupt is generated if the STRBERRIE bit is set to 1. Writing 1 to CLR_STRBERR bit in FLASH_SECCCR register clears STRBERR."]
    #[inline(always)]
    pub fn strberr(&self) -> STRBERR_R {
        STRBERR_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - inconsistency error flag INCERR flag is raised when a inconsistency error occurs. An interrupt is generated if INCERRIE is set to 1. Writing 1 to CLR_INCERR bit in the FLASH_SECCCR register clears INCERR."]
    #[inline(always)]
    pub fn incerr(&self) -> INCERR_R {
        INCERR_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - OBK general error flag OBKERR flag is raised when the OBK-HDPL signal from the SBS does not match the HDPL value associated with the key slot during access to the key location. Alternatively also when the ALT_SECT is unexpectedly changed while the write buffer is being filled."]
    #[inline(always)]
    pub fn obkerr(&self) -> OBKERR_R {
        OBKERR_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - OBK write error flag OBKWERR flag is raised when the address is not virgin on a write access to the OBK storage. Alternatively also when the OBK selector in the alternate sector is not virgin during a swap operation."]
    #[inline(always)]
    pub fn obkwerr(&self) -> OBKWERR_R {
        OBKWERR_R::new(((self.bits >> 22) & 1) != 0)
    }
}
#[doc = "FLASH secure status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [secsr](index.html) module"]
pub struct SECSR_SPEC;
impl crate::RegisterSpec for SECSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [secsr::R](R) reader structure"]
impl crate::Readable for SECSR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets SECSR to value 0"]
impl crate::Resettable for SECSR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
