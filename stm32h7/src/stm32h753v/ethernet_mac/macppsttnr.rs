#[doc = "Register `MACPPSTTNR` reader"]
pub struct R(crate::R<MACPPSTTNR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MACPPSTTNR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MACPPSTTNR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MACPPSTTNR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MACPPSTTNR` writer"]
pub struct W(crate::W<MACPPSTTNR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MACPPSTTNR_SPEC>;
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
impl From<crate::W<MACPPSTTNR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MACPPSTTNR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TTSL0` reader - Target Time Low for PPS Register"]
pub type TTSL0_R = crate::FieldReader<u32, u32>;
#[doc = "Field `TTSL0` writer - Target Time Low for PPS Register"]
pub type TTSL0_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MACPPSTTNR_SPEC, u32, u32, 31, O>;
#[doc = "Field `TRGTBUSY0` reader - PPS Target Time Register Busy"]
pub type TRGTBUSY0_R = crate::BitReader<bool>;
#[doc = "Field `TRGTBUSY0` writer - PPS Target Time Register Busy"]
pub type TRGTBUSY0_W<'a, const O: u8> = crate::BitWriter<'a, u32, MACPPSTTNR_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:30 - Target Time Low for PPS Register"]
    #[inline(always)]
    pub fn ttsl0(&self) -> TTSL0_R {
        TTSL0_R::new(self.bits & 0x7fff_ffff)
    }
    #[doc = "Bit 31 - PPS Target Time Register Busy"]
    #[inline(always)]
    pub fn trgtbusy0(&self) -> TRGTBUSY0_R {
        TRGTBUSY0_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:30 - Target Time Low for PPS Register"]
    #[inline(always)]
    #[must_use]
    pub fn ttsl0(&mut self) -> TTSL0_W<0> {
        TTSL0_W::new(self)
    }
    #[doc = "Bit 31 - PPS Target Time Register Busy"]
    #[inline(always)]
    #[must_use]
    pub fn trgtbusy0(&mut self) -> TRGTBUSY0_W<31> {
        TRGTBUSY0_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PPS target time nanoseconds register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [macppsttnr](index.html) module"]
pub struct MACPPSTTNR_SPEC;
impl crate::RegisterSpec for MACPPSTTNR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [macppsttnr::R](R) reader structure"]
impl crate::Readable for MACPPSTTNR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [macppsttnr::W](W) writer structure"]
impl crate::Writable for MACPPSTTNR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MACPPSTTNR to value 0"]
impl crate::Resettable for MACPPSTTNR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
