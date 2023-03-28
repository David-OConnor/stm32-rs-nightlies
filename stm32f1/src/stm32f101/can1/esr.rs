#[doc = "Register `ESR` reader"]
pub struct R(crate::R<ESR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ESR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ESR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ESR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ESR` writer"]
pub struct W(crate::W<ESR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ESR_SPEC>;
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
impl From<crate::W<ESR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ESR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EWGF` reader - EWGF"]
pub type EWGF_R = crate::BitReader<bool>;
#[doc = "Field `EPVF` reader - EPVF"]
pub type EPVF_R = crate::BitReader<bool>;
#[doc = "Field `BOFF` reader - BOFF"]
pub type BOFF_R = crate::BitReader<bool>;
#[doc = "Field `LEC` reader - LEC"]
pub type LEC_R = crate::FieldReader<u8, u8>;
#[doc = "Field `LEC` writer - LEC"]
pub type LEC_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ESR_SPEC, u8, u8, 3, O>;
#[doc = "Field `TEC` reader - TEC"]
pub type TEC_R = crate::FieldReader<u8, u8>;
#[doc = "Field `REC` reader - REC"]
pub type REC_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bit 0 - EWGF"]
    #[inline(always)]
    pub fn ewgf(&self) -> EWGF_R {
        EWGF_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - EPVF"]
    #[inline(always)]
    pub fn epvf(&self) -> EPVF_R {
        EPVF_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - BOFF"]
    #[inline(always)]
    pub fn boff(&self) -> BOFF_R {
        BOFF_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 4:6 - LEC"]
    #[inline(always)]
    pub fn lec(&self) -> LEC_R {
        LEC_R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bits 16:23 - TEC"]
    #[inline(always)]
    pub fn tec(&self) -> TEC_R {
        TEC_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - REC"]
    #[inline(always)]
    pub fn rec(&self) -> REC_R {
        REC_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 4:6 - LEC"]
    #[inline(always)]
    #[must_use]
    pub fn lec(&mut self) -> LEC_W<4> {
        LEC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "error status register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [esr](index.html) module"]
pub struct ESR_SPEC;
impl crate::RegisterSpec for ESR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [esr::R](R) reader structure"]
impl crate::Readable for ESR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [esr::W](W) writer structure"]
impl crate::Writable for ESR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ESR to value 0"]
impl crate::Resettable for ESR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
