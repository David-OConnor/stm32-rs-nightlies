#[doc = "Register `RIR` reader"]
pub struct R(crate::R<RIR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RIR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RIR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RIR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `RTR` reader - RTR"]
pub type RTR_R = crate::BitReader<bool>;
#[doc = "Field `IDE` reader - IDE"]
pub type IDE_R = crate::BitReader<bool>;
#[doc = "Field `EXID` reader - EXID"]
pub type EXID_R = crate::FieldReader<u32, u32>;
#[doc = "Field `STID` reader - STID"]
pub type STID_R = crate::FieldReader<u16, u16>;
impl R {
    #[doc = "Bit 1 - RTR"]
    #[inline(always)]
    pub fn rtr(&self) -> RTR_R {
        RTR_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - IDE"]
    #[inline(always)]
    pub fn ide(&self) -> IDE_R {
        IDE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:20 - EXID"]
    #[inline(always)]
    pub fn exid(&self) -> EXID_R {
        EXID_R::new((self.bits >> 3) & 0x0003_ffff)
    }
    #[doc = "Bits 21:31 - STID"]
    #[inline(always)]
    pub fn stid(&self) -> STID_R {
        STID_R::new(((self.bits >> 21) & 0x07ff) as u16)
    }
}
#[doc = "receive FIFO mailbox identifier register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rir](index.html) module"]
pub struct RIR_SPEC;
impl crate::RegisterSpec for RIR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rir::R](R) reader structure"]
impl crate::Readable for RIR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets RIR to value 0"]
impl crate::Resettable for RIR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
