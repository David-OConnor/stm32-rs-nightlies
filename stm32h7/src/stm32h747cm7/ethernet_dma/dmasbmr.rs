#[doc = "Register `DMASBMR` reader"]
pub struct R(crate::R<DMASBMR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DMASBMR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DMASBMR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DMASBMR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DMASBMR` writer"]
pub struct W(crate::W<DMASBMR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DMASBMR_SPEC>;
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
impl From<crate::W<DMASBMR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DMASBMR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FB` reader - Fixed Burst Length"]
pub type FB_R = crate::BitReader<bool>;
#[doc = "Field `FB` writer - Fixed Burst Length"]
pub type FB_W<'a, const O: u8> = crate::BitWriter<'a, u32, DMASBMR_SPEC, bool, O>;
#[doc = "Field `AAL` reader - Address-Aligned Beats"]
pub type AAL_R = crate::BitReader<bool>;
#[doc = "Field `AAL` writer - Address-Aligned Beats"]
pub type AAL_W<'a, const O: u8> = crate::BitWriter<'a, u32, DMASBMR_SPEC, bool, O>;
#[doc = "Field `MB` reader - Mixed Burst"]
pub type MB_R = crate::BitReader<bool>;
#[doc = "Field `MB` writer - Mixed Burst"]
pub type MB_W<'a, const O: u8> = crate::BitWriter<'a, u32, DMASBMR_SPEC, bool, O>;
#[doc = "Field `RB` reader - Rebuild INCRx Burst"]
pub type RB_R = crate::BitReader<bool>;
#[doc = "Field `RB` writer - Rebuild INCRx Burst"]
pub type RB_W<'a, const O: u8> = crate::BitWriter<'a, u32, DMASBMR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Fixed Burst Length"]
    #[inline(always)]
    pub fn fb(&self) -> FB_R {
        FB_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 12 - Address-Aligned Beats"]
    #[inline(always)]
    pub fn aal(&self) -> AAL_R {
        AAL_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 14 - Mixed Burst"]
    #[inline(always)]
    pub fn mb(&self) -> MB_R {
        MB_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Rebuild INCRx Burst"]
    #[inline(always)]
    pub fn rb(&self) -> RB_R {
        RB_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Fixed Burst Length"]
    #[inline(always)]
    #[must_use]
    pub fn fb(&mut self) -> FB_W<0> {
        FB_W::new(self)
    }
    #[doc = "Bit 12 - Address-Aligned Beats"]
    #[inline(always)]
    #[must_use]
    pub fn aal(&mut self) -> AAL_W<12> {
        AAL_W::new(self)
    }
    #[doc = "Bit 14 - Mixed Burst"]
    #[inline(always)]
    #[must_use]
    pub fn mb(&mut self) -> MB_W<14> {
        MB_W::new(self)
    }
    #[doc = "Bit 15 - Rebuild INCRx Burst"]
    #[inline(always)]
    #[must_use]
    pub fn rb(&mut self) -> RB_W<15> {
        RB_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "System bus mode register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dmasbmr](index.html) module"]
pub struct DMASBMR_SPEC;
impl crate::RegisterSpec for DMASBMR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dmasbmr::R](R) reader structure"]
impl crate::Readable for DMASBMR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dmasbmr::W](W) writer structure"]
impl crate::Writable for DMASBMR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DMASBMR to value 0x0101_0000"]
impl crate::Resettable for DMASBMR_SPEC {
    const RESET_VALUE: Self::Ux = 0x0101_0000;
}
