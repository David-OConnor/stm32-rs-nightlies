#[doc = "Register `ECCDETR` reader"]
pub struct R(crate::R<ECCDETR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ECCDETR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ECCDETR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ECCDETR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ECCDETR` writer"]
pub struct W(crate::W<ECCDETR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ECCDETR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<ECCDETR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ECCDETR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ADDR_ECC` reader - ECC error address When an ECC error occurs (double detection) during a read operation, the ADDR_ECC contains the address that generated the error. ADDR_ECC is reset when the flag error is reset. The flash interface programs the address in this register only when no ECC error flags are set. This means that only the first address that generated an double ECC error is saved. The address in ADDR_ECC is relative to the flash memory area where the error occurred (user flash memory, system flash memory, data area, read-only/OTP area)."]
pub type ADDR_ECC_R = crate::FieldReader<u16, u16>;
#[doc = "Field `OBK_ECC` reader - ECC fail double ECC error in flash OB Keys storage area. It indicates the OBK storage concerned by ECC error."]
pub type OBK_ECC_R = crate::BitReader<bool>;
#[doc = "Field `EDATA_ECC` reader - ECC fail double ECC error in flash high-cycle data area It indicates if flash high-cycle data area is concerned by ECC error."]
pub type EDATA_ECC_R = crate::BitReader<bool>;
#[doc = "Field `BK_ECC` reader - ECC fail bank for double ECC error It indicates which bank is concerned by ECC error"]
pub type BK_ECC_R = crate::BitReader<bool>;
#[doc = "Field `SYSF_ECC` reader - ECC fail for double ECC error in system flash memory It indicates if system flash memory is concerned by ECC error."]
pub type SYSF_ECC_R = crate::BitReader<bool>;
#[doc = "Field `OTP_ECC` reader - OTP ECC error bit This bit is set to 1 when double ECC detection occurred during the last read operation from the read-only/ OTP area. The address of the ECC error is available in ADDR_ECC bitfield."]
pub type OTP_ECC_R = crate::BitReader<bool>;
#[doc = "Field `ECCD` reader - ECC detection Set by hardware when two ECC error has been detected. When this bit is set, a NMI is generated. Cleared by writing 1. Needs to be cleared in order to detect subsequent double ECC errors."]
pub type ECCD_R = crate::BitReader<bool>;
#[doc = "Field `ECCD` writer - ECC detection Set by hardware when two ECC error has been detected. When this bit is set, a NMI is generated. Cleared by writing 1. Needs to be cleared in order to detect subsequent double ECC errors."]
pub type ECCD_W<'a, const O: u8> = crate::BitWriter<'a, u32, ECCDETR_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:15 - ECC error address When an ECC error occurs (double detection) during a read operation, the ADDR_ECC contains the address that generated the error. ADDR_ECC is reset when the flag error is reset. The flash interface programs the address in this register only when no ECC error flags are set. This means that only the first address that generated an double ECC error is saved. The address in ADDR_ECC is relative to the flash memory area where the error occurred (user flash memory, system flash memory, data area, read-only/OTP area)."]
    #[inline(always)]
    pub fn addr_ecc(&self) -> ADDR_ECC_R {
        ADDR_ECC_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bit 20 - ECC fail double ECC error in flash OB Keys storage area. It indicates the OBK storage concerned by ECC error."]
    #[inline(always)]
    pub fn obk_ecc(&self) -> OBK_ECC_R {
        OBK_ECC_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - ECC fail double ECC error in flash high-cycle data area It indicates if flash high-cycle data area is concerned by ECC error."]
    #[inline(always)]
    pub fn edata_ecc(&self) -> EDATA_ECC_R {
        EDATA_ECC_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - ECC fail bank for double ECC error It indicates which bank is concerned by ECC error"]
    #[inline(always)]
    pub fn bk_ecc(&self) -> BK_ECC_R {
        BK_ECC_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - ECC fail for double ECC error in system flash memory It indicates if system flash memory is concerned by ECC error."]
    #[inline(always)]
    pub fn sysf_ecc(&self) -> SYSF_ECC_R {
        SYSF_ECC_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - OTP ECC error bit This bit is set to 1 when double ECC detection occurred during the last read operation from the read-only/ OTP area. The address of the ECC error is available in ADDR_ECC bitfield."]
    #[inline(always)]
    pub fn otp_ecc(&self) -> OTP_ECC_R {
        OTP_ECC_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 31 - ECC detection Set by hardware when two ECC error has been detected. When this bit is set, a NMI is generated. Cleared by writing 1. Needs to be cleared in order to detect subsequent double ECC errors."]
    #[inline(always)]
    pub fn eccd(&self) -> ECCD_R {
        ECCD_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 31 - ECC detection Set by hardware when two ECC error has been detected. When this bit is set, a NMI is generated. Cleared by writing 1. Needs to be cleared in order to detect subsequent double ECC errors."]
    #[inline(always)]
    #[must_use]
    pub fn eccd(&mut self) -> ECCD_W<31> {
        ECCD_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "FLASH ECC detection register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [eccdetr](index.html) module"]
pub struct ECCDETR_SPEC;
impl crate::RegisterSpec for ECCDETR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [eccdetr::R](R) reader structure"]
impl crate::Readable for ECCDETR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [eccdetr::W](W) writer structure"]
impl crate::Writable for ECCDETR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ECCDETR to value 0"]
impl crate::Resettable for ECCDETR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
