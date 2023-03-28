#[doc = "Register `IDMACTRLR` reader"]
pub struct R(crate::R<IDMACTRLR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IDMACTRLR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IDMACTRLR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IDMACTRLR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `IDMACTRLR` writer"]
pub struct W(crate::W<IDMACTRLR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IDMACTRLR_SPEC>;
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
impl From<crate::W<IDMACTRLR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IDMACTRLR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `IDMAEN` reader - IDMA enable"]
pub type IDMAEN_R = crate::BitReader<bool>;
#[doc = "Field `IDMAEN` writer - IDMA enable"]
pub type IDMAEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, IDMACTRLR_SPEC, bool, O>;
#[doc = "Field `IDMABMODE` reader - Buffer mode selection"]
pub type IDMABMODE_R = crate::BitReader<bool>;
#[doc = "Field `IDMABMODE` writer - Buffer mode selection"]
pub type IDMABMODE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IDMACTRLR_SPEC, bool, O>;
#[doc = "Field `IDMABACT` reader - Double buffer mode active buffer indication"]
pub type IDMABACT_R = crate::BitReader<bool>;
#[doc = "Field `IDMABACT` writer - Double buffer mode active buffer indication"]
pub type IDMABACT_W<'a, const O: u8> = crate::BitWriter<'a, u32, IDMACTRLR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - IDMA enable"]
    #[inline(always)]
    pub fn idmaen(&self) -> IDMAEN_R {
        IDMAEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Buffer mode selection"]
    #[inline(always)]
    pub fn idmabmode(&self) -> IDMABMODE_R {
        IDMABMODE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Double buffer mode active buffer indication"]
    #[inline(always)]
    pub fn idmabact(&self) -> IDMABACT_R {
        IDMABACT_R::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - IDMA enable"]
    #[inline(always)]
    #[must_use]
    pub fn idmaen(&mut self) -> IDMAEN_W<0> {
        IDMAEN_W::new(self)
    }
    #[doc = "Bit 1 - Buffer mode selection"]
    #[inline(always)]
    #[must_use]
    pub fn idmabmode(&mut self) -> IDMABMODE_W<1> {
        IDMABMODE_W::new(self)
    }
    #[doc = "Bit 2 - Double buffer mode active buffer indication"]
    #[inline(always)]
    #[must_use]
    pub fn idmabact(&mut self) -> IDMABACT_W<2> {
        IDMABACT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DMA control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [idmactrlr](index.html) module"]
pub struct IDMACTRLR_SPEC;
impl crate::RegisterSpec for IDMACTRLR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [idmactrlr::R](R) reader structure"]
impl crate::Readable for IDMACTRLR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [idmactrlr::W](W) writer structure"]
impl crate::Writable for IDMACTRLR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets IDMACTRLR to value 0"]
impl crate::Resettable for IDMACTRLR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
