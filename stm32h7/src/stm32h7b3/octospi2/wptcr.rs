#[doc = "Register `WPTCR` reader"]
pub struct R(crate::R<WPTCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<WPTCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<WPTCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<WPTCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `WPTCR` writer"]
pub struct W(crate::W<WPTCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<WPTCR_SPEC>;
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
impl From<crate::W<WPTCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<WPTCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DCYC` reader - Number of dummy cycles"]
pub type DCYC_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DCYC` writer - Number of dummy cycles"]
pub type DCYC_W<'a, const O: u8> = crate::FieldWriter<'a, u32, WPTCR_SPEC, u8, u8, 5, O>;
#[doc = "Field `DHQC` reader - Delay hold quarter cycle"]
pub type DHQC_R = crate::BitReader<bool>;
#[doc = "Field `DHQC` writer - Delay hold quarter cycle"]
pub type DHQC_W<'a, const O: u8> = crate::BitWriter<'a, u32, WPTCR_SPEC, bool, O>;
#[doc = "Field `SSHIFT` reader - Sample shift"]
pub type SSHIFT_R = crate::BitReader<bool>;
#[doc = "Field `SSHIFT` writer - Sample shift"]
pub type SSHIFT_W<'a, const O: u8> = crate::BitWriter<'a, u32, WPTCR_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:4 - Number of dummy cycles"]
    #[inline(always)]
    pub fn dcyc(&self) -> DCYC_R {
        DCYC_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bit 28 - Delay hold quarter cycle"]
    #[inline(always)]
    pub fn dhqc(&self) -> DHQC_R {
        DHQC_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 30 - Sample shift"]
    #[inline(always)]
    pub fn sshift(&self) -> SSHIFT_R {
        SSHIFT_R::new(((self.bits >> 30) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:4 - Number of dummy cycles"]
    #[inline(always)]
    #[must_use]
    pub fn dcyc(&mut self) -> DCYC_W<0> {
        DCYC_W::new(self)
    }
    #[doc = "Bit 28 - Delay hold quarter cycle"]
    #[inline(always)]
    #[must_use]
    pub fn dhqc(&mut self) -> DHQC_W<28> {
        DHQC_W::new(self)
    }
    #[doc = "Bit 30 - Sample shift"]
    #[inline(always)]
    #[must_use]
    pub fn sshift(&mut self) -> SSHIFT_W<30> {
        SSHIFT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "wrap timing configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wptcr](index.html) module"]
pub struct WPTCR_SPEC;
impl crate::RegisterSpec for WPTCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [wptcr::R](R) reader structure"]
impl crate::Readable for WPTCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [wptcr::W](W) writer structure"]
impl crate::Writable for WPTCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets WPTCR to value 0"]
impl crate::Resettable for WPTCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
