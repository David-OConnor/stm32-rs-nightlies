#[doc = "Register `DDRPHYC_ODTCR` reader"]
pub struct R(crate::R<DDRPHYC_ODTCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DDRPHYC_ODTCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DDRPHYC_ODTCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DDRPHYC_ODTCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DDRPHYC_ODTCR` writer"]
pub struct W(crate::W<DDRPHYC_ODTCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DDRPHYC_ODTCR_SPEC>;
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
impl From<crate::W<DDRPHYC_ODTCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DDRPHYC_ODTCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RDODT` reader - RDODT"]
pub type RDODT_R = crate::BitReader<bool>;
#[doc = "Field `RDODT` writer - RDODT"]
pub type RDODT_W<'a, const O: u8> = crate::BitWriter<'a, u32, DDRPHYC_ODTCR_SPEC, bool, O>;
#[doc = "Field `WRODT` reader - WRODT"]
pub type WRODT_R = crate::BitReader<bool>;
#[doc = "Field `WRODT` writer - WRODT"]
pub type WRODT_W<'a, const O: u8> = crate::BitWriter<'a, u32, DDRPHYC_ODTCR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - RDODT"]
    #[inline(always)]
    pub fn rdodt(&self) -> RDODT_R {
        RDODT_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 16 - WRODT"]
    #[inline(always)]
    pub fn wrodt(&self) -> WRODT_R {
        WRODT_R::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - RDODT"]
    #[inline(always)]
    #[must_use]
    pub fn rdodt(&mut self) -> RDODT_W<0> {
        RDODT_W::new(self)
    }
    #[doc = "Bit 16 - WRODT"]
    #[inline(always)]
    #[must_use]
    pub fn wrodt(&mut self) -> WRODT_W<16> {
        WRODT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DDRPHYC ODTC register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ddrphyc_odtcr](index.html) module"]
pub struct DDRPHYC_ODTCR_SPEC;
impl crate::RegisterSpec for DDRPHYC_ODTCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ddrphyc_odtcr::R](R) reader structure"]
impl crate::Readable for DDRPHYC_ODTCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ddrphyc_odtcr::W](W) writer structure"]
impl crate::Writable for DDRPHYC_ODTCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DDRPHYC_ODTCR to value 0x8421_0000"]
impl crate::Resettable for DDRPHYC_ODTCR_SPEC {
    const RESET_VALUE: Self::Ux = 0x8421_0000;
}
