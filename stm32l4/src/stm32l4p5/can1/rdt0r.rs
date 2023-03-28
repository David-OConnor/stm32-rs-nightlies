#[doc = "Register `RDT0R` reader"]
pub struct R(crate::R<RDT0R_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RDT0R_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RDT0R_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RDT0R_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `DLC` reader - DLC"]
pub type DLC_R = crate::FieldReader<u8, u8>;
#[doc = "Field `FMI` reader - FMI"]
pub type FMI_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TIME` reader - TIME"]
pub type TIME_R = crate::FieldReader<u16, u16>;
impl R {
    #[doc = "Bits 0:3 - DLC"]
    #[inline(always)]
    pub fn dlc(&self) -> DLC_R {
        DLC_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 8:15 - FMI"]
    #[inline(always)]
    pub fn fmi(&self) -> FMI_R {
        FMI_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:31 - TIME"]
    #[inline(always)]
    pub fn time(&self) -> TIME_R {
        TIME_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
#[doc = "mailbox data high register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rdt0r](index.html) module"]
pub struct RDT0R_SPEC;
impl crate::RegisterSpec for RDT0R_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rdt0r::R](R) reader structure"]
impl crate::Readable for RDT0R_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets RDT0R to value 0"]
impl crate::Resettable for RDT0R_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
