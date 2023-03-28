#[doc = "Register `IOPENR` reader"]
pub struct R(crate::R<IOPENR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IOPENR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IOPENR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IOPENR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `IOPENR` writer"]
pub struct W(crate::W<IOPENR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IOPENR_SPEC>;
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
impl From<crate::W<IOPENR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IOPENR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `IOPAEN` reader - I/O port A clock enable"]
pub type IOPAEN_R = crate::BitReader<bool>;
#[doc = "Field `IOPAEN` writer - I/O port A clock enable"]
pub type IOPAEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, IOPENR_SPEC, bool, O>;
#[doc = "Field `IOPBEN` reader - I/O port B clock enable"]
pub type IOPBEN_R = crate::BitReader<bool>;
#[doc = "Field `IOPBEN` writer - I/O port B clock enable"]
pub type IOPBEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, IOPENR_SPEC, bool, O>;
#[doc = "Field `IOPCEN` reader - I/O port C clock enable"]
pub type IOPCEN_R = crate::BitReader<bool>;
#[doc = "Field `IOPCEN` writer - I/O port C clock enable"]
pub type IOPCEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, IOPENR_SPEC, bool, O>;
#[doc = "Field `IOPDEN` reader - I/O port D clock enable"]
pub type IOPDEN_R = crate::BitReader<bool>;
#[doc = "Field `IOPDEN` writer - I/O port D clock enable"]
pub type IOPDEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, IOPENR_SPEC, bool, O>;
#[doc = "Field `IOPFEN` reader - I/O port F clock enable"]
pub type IOPFEN_R = crate::BitReader<bool>;
#[doc = "Field `IOPFEN` writer - I/O port F clock enable"]
pub type IOPFEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, IOPENR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - I/O port A clock enable"]
    #[inline(always)]
    pub fn iopaen(&self) -> IOPAEN_R {
        IOPAEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - I/O port B clock enable"]
    #[inline(always)]
    pub fn iopben(&self) -> IOPBEN_R {
        IOPBEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - I/O port C clock enable"]
    #[inline(always)]
    pub fn iopcen(&self) -> IOPCEN_R {
        IOPCEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - I/O port D clock enable"]
    #[inline(always)]
    pub fn iopden(&self) -> IOPDEN_R {
        IOPDEN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 5 - I/O port F clock enable"]
    #[inline(always)]
    pub fn iopfen(&self) -> IOPFEN_R {
        IOPFEN_R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - I/O port A clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn iopaen(&mut self) -> IOPAEN_W<0> {
        IOPAEN_W::new(self)
    }
    #[doc = "Bit 1 - I/O port B clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn iopben(&mut self) -> IOPBEN_W<1> {
        IOPBEN_W::new(self)
    }
    #[doc = "Bit 2 - I/O port C clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn iopcen(&mut self) -> IOPCEN_W<2> {
        IOPCEN_W::new(self)
    }
    #[doc = "Bit 3 - I/O port D clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn iopden(&mut self) -> IOPDEN_W<3> {
        IOPDEN_W::new(self)
    }
    #[doc = "Bit 5 - I/O port F clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn iopfen(&mut self) -> IOPFEN_W<5> {
        IOPFEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "GPIO clock enable register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [iopenr](index.html) module"]
pub struct IOPENR_SPEC;
impl crate::RegisterSpec for IOPENR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [iopenr::R](R) reader structure"]
impl crate::Readable for IOPENR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [iopenr::W](W) writer structure"]
impl crate::Writable for IOPENR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets IOPENR to value 0"]
impl crate::Resettable for IOPENR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
