#[doc = "Register `SR` reader"]
pub struct R(crate::R<SR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `PERF` reader - Preamble error flag"]
pub type PERF_R = crate::BitReader<bool>;
#[doc = "Field `SERF` reader - Start error flag"]
pub type SERF_R = crate::BitReader<bool>;
#[doc = "Field `TERF` reader - Turnaround error flag"]
pub type TERF_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0 - Preamble error flag"]
    #[inline(always)]
    pub fn perf(&self) -> PERF_R {
        PERF_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Start error flag"]
    #[inline(always)]
    pub fn serf(&self) -> SERF_R {
        SERF_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Turnaround error flag"]
    #[inline(always)]
    pub fn terf(&self) -> TERF_R {
        TERF_R::new(((self.bits >> 2) & 1) != 0)
    }
}
#[doc = "MDIOS status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sr](index.html) module"]
pub struct SR_SPEC;
impl crate::RegisterSpec for SR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sr::R](R) reader structure"]
impl crate::Readable for SR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets SR to value 0"]
impl crate::Resettable for SR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
