#[doc = "Register `ABFSR` reader"]
pub struct R(crate::R<ABFSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ABFSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ABFSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ABFSR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ABFSR` writer"]
pub struct W(crate::W<ABFSR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ABFSR_SPEC>;
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
impl From<crate::W<ABFSR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ABFSR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ITCM` reader - ITCM"]
pub type ITCM_R = crate::BitReader<bool>;
#[doc = "Field `ITCM` writer - ITCM"]
pub type ITCM_W<'a, const O: u8> = crate::BitWriter<'a, u32, ABFSR_SPEC, bool, O>;
#[doc = "Field `DTCM` reader - DTCM"]
pub type DTCM_R = crate::BitReader<bool>;
#[doc = "Field `DTCM` writer - DTCM"]
pub type DTCM_W<'a, const O: u8> = crate::BitWriter<'a, u32, ABFSR_SPEC, bool, O>;
#[doc = "Field `AHBP` reader - AHBP"]
pub type AHBP_R = crate::BitReader<bool>;
#[doc = "Field `AHBP` writer - AHBP"]
pub type AHBP_W<'a, const O: u8> = crate::BitWriter<'a, u32, ABFSR_SPEC, bool, O>;
#[doc = "Field `AXIM` reader - AXIM"]
pub type AXIM_R = crate::BitReader<bool>;
#[doc = "Field `AXIM` writer - AXIM"]
pub type AXIM_W<'a, const O: u8> = crate::BitWriter<'a, u32, ABFSR_SPEC, bool, O>;
#[doc = "Field `EPPB` reader - EPPB"]
pub type EPPB_R = crate::BitReader<bool>;
#[doc = "Field `EPPB` writer - EPPB"]
pub type EPPB_W<'a, const O: u8> = crate::BitWriter<'a, u32, ABFSR_SPEC, bool, O>;
#[doc = "Field `AXIMTYPE` reader - AXIMTYPE"]
pub type AXIMTYPE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `AXIMTYPE` writer - AXIMTYPE"]
pub type AXIMTYPE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ABFSR_SPEC, u8, u8, 2, O>;
impl R {
    #[doc = "Bit 0 - ITCM"]
    #[inline(always)]
    pub fn itcm(&self) -> ITCM_R {
        ITCM_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - DTCM"]
    #[inline(always)]
    pub fn dtcm(&self) -> DTCM_R {
        DTCM_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - AHBP"]
    #[inline(always)]
    pub fn ahbp(&self) -> AHBP_R {
        AHBP_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - AXIM"]
    #[inline(always)]
    pub fn axim(&self) -> AXIM_R {
        AXIM_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - EPPB"]
    #[inline(always)]
    pub fn eppb(&self) -> EPPB_R {
        EPPB_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 8:9 - AXIMTYPE"]
    #[inline(always)]
    pub fn aximtype(&self) -> AXIMTYPE_R {
        AXIMTYPE_R::new(((self.bits >> 8) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - ITCM"]
    #[inline(always)]
    #[must_use]
    pub fn itcm(&mut self) -> ITCM_W<0> {
        ITCM_W::new(self)
    }
    #[doc = "Bit 1 - DTCM"]
    #[inline(always)]
    #[must_use]
    pub fn dtcm(&mut self) -> DTCM_W<1> {
        DTCM_W::new(self)
    }
    #[doc = "Bit 2 - AHBP"]
    #[inline(always)]
    #[must_use]
    pub fn ahbp(&mut self) -> AHBP_W<2> {
        AHBP_W::new(self)
    }
    #[doc = "Bit 3 - AXIM"]
    #[inline(always)]
    #[must_use]
    pub fn axim(&mut self) -> AXIM_W<3> {
        AXIM_W::new(self)
    }
    #[doc = "Bit 4 - EPPB"]
    #[inline(always)]
    #[must_use]
    pub fn eppb(&mut self) -> EPPB_W<4> {
        EPPB_W::new(self)
    }
    #[doc = "Bits 8:9 - AXIMTYPE"]
    #[inline(always)]
    #[must_use]
    pub fn aximtype(&mut self) -> AXIMTYPE_W<8> {
        AXIMTYPE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Auxiliary Bus Fault Status register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [abfsr](index.html) module"]
pub struct ABFSR_SPEC;
impl crate::RegisterSpec for ABFSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [abfsr::R](R) reader structure"]
impl crate::Readable for ABFSR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [abfsr::W](W) writer structure"]
impl crate::Writable for ABFSR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ABFSR to value 0"]
impl crate::Resettable for ABFSR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
