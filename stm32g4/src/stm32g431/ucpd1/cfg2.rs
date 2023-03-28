#[doc = "Register `CFG2` reader"]
pub struct R(crate::R<CFG2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CFG2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CFG2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CFG2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CFG2` writer"]
pub struct W(crate::W<CFG2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CFG2_SPEC>;
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
impl From<crate::W<CFG2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CFG2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RXFILTDIS` reader - RXFILTDIS"]
pub type RXFILTDIS_R = crate::BitReader<bool>;
#[doc = "Field `RXFILTDIS` writer - RXFILTDIS"]
pub type RXFILTDIS_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFG2_SPEC, bool, O>;
#[doc = "Field `RXFILT2N3` reader - RXFILT2N3"]
pub type RXFILT2N3_R = crate::BitReader<bool>;
#[doc = "Field `RXFILT2N3` writer - RXFILT2N3"]
pub type RXFILT2N3_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFG2_SPEC, bool, O>;
#[doc = "Field `FORCECLK` reader - FORCECLK"]
pub type FORCECLK_R = crate::BitReader<bool>;
#[doc = "Field `FORCECLK` writer - FORCECLK"]
pub type FORCECLK_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFG2_SPEC, bool, O>;
#[doc = "Field `WUPEN` reader - WUPEN"]
pub type WUPEN_R = crate::BitReader<bool>;
#[doc = "Field `WUPEN` writer - WUPEN"]
pub type WUPEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFG2_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - RXFILTDIS"]
    #[inline(always)]
    pub fn rxfiltdis(&self) -> RXFILTDIS_R {
        RXFILTDIS_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - RXFILT2N3"]
    #[inline(always)]
    pub fn rxfilt2n3(&self) -> RXFILT2N3_R {
        RXFILT2N3_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - FORCECLK"]
    #[inline(always)]
    pub fn forceclk(&self) -> FORCECLK_R {
        FORCECLK_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - WUPEN"]
    #[inline(always)]
    pub fn wupen(&self) -> WUPEN_R {
        WUPEN_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - RXFILTDIS"]
    #[inline(always)]
    #[must_use]
    pub fn rxfiltdis(&mut self) -> RXFILTDIS_W<0> {
        RXFILTDIS_W::new(self)
    }
    #[doc = "Bit 1 - RXFILT2N3"]
    #[inline(always)]
    #[must_use]
    pub fn rxfilt2n3(&mut self) -> RXFILT2N3_W<1> {
        RXFILT2N3_W::new(self)
    }
    #[doc = "Bit 2 - FORCECLK"]
    #[inline(always)]
    #[must_use]
    pub fn forceclk(&mut self) -> FORCECLK_W<2> {
        FORCECLK_W::new(self)
    }
    #[doc = "Bit 3 - WUPEN"]
    #[inline(always)]
    #[must_use]
    pub fn wupen(&mut self) -> WUPEN_W<3> {
        WUPEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "UCPD configuration register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cfg2](index.html) module"]
pub struct CFG2_SPEC;
impl crate::RegisterSpec for CFG2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cfg2::R](R) reader structure"]
impl crate::Readable for CFG2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cfg2::W](W) writer structure"]
impl crate::Writable for CFG2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CFG2 to value 0"]
impl crate::Resettable for CFG2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
