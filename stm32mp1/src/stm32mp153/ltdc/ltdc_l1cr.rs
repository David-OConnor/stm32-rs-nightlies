#[doc = "Register `LTDC_L1CR` reader"]
pub struct R(crate::R<LTDC_L1CR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LTDC_L1CR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LTDC_L1CR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LTDC_L1CR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `LTDC_L1CR` writer"]
pub struct W(crate::W<LTDC_L1CR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LTDC_L1CR_SPEC>;
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
impl From<crate::W<LTDC_L1CR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LTDC_L1CR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LEN` reader - LEN"]
pub type LEN_R = crate::BitReader<bool>;
#[doc = "Field `LEN` writer - LEN"]
pub type LEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, LTDC_L1CR_SPEC, bool, O>;
#[doc = "Field `COLKEN` reader - COLKEN"]
pub type COLKEN_R = crate::BitReader<bool>;
#[doc = "Field `COLKEN` writer - COLKEN"]
pub type COLKEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, LTDC_L1CR_SPEC, bool, O>;
#[doc = "Field `CLUTEN` reader - CLUTEN"]
pub type CLUTEN_R = crate::BitReader<bool>;
#[doc = "Field `CLUTEN` writer - CLUTEN"]
pub type CLUTEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, LTDC_L1CR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - LEN"]
    #[inline(always)]
    pub fn len(&self) -> LEN_R {
        LEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - COLKEN"]
    #[inline(always)]
    pub fn colken(&self) -> COLKEN_R {
        COLKEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 4 - CLUTEN"]
    #[inline(always)]
    pub fn cluten(&self) -> CLUTEN_R {
        CLUTEN_R::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - LEN"]
    #[inline(always)]
    #[must_use]
    pub fn len(&mut self) -> LEN_W<0> {
        LEN_W::new(self)
    }
    #[doc = "Bit 1 - COLKEN"]
    #[inline(always)]
    #[must_use]
    pub fn colken(&mut self) -> COLKEN_W<1> {
        COLKEN_W::new(self)
    }
    #[doc = "Bit 4 - CLUTEN"]
    #[inline(always)]
    #[must_use]
    pub fn cluten(&mut self) -> CLUTEN_W<4> {
        CLUTEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "LTDC layer 1 control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltdc_l1cr](index.html) module"]
pub struct LTDC_L1CR_SPEC;
impl crate::RegisterSpec for LTDC_L1CR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ltdc_l1cr::R](R) reader structure"]
impl crate::Readable for LTDC_L1CR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ltdc_l1cr::W](W) writer structure"]
impl crate::Writable for LTDC_L1CR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets LTDC_L1CR to value 0"]
impl crate::Resettable for LTDC_L1CR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
