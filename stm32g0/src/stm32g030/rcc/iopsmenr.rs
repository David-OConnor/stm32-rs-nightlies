#[doc = "Register `IOPSMENR` reader"]
pub struct R(crate::R<IOPSMENR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IOPSMENR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IOPSMENR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IOPSMENR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `IOPSMENR` writer"]
pub struct W(crate::W<IOPSMENR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IOPSMENR_SPEC>;
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
impl From<crate::W<IOPSMENR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IOPSMENR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `IOPASMEN` reader - I/O port A clock enable during Sleep mode"]
pub type IOPASMEN_R = crate::BitReader<bool>;
#[doc = "Field `IOPASMEN` writer - I/O port A clock enable during Sleep mode"]
pub type IOPASMEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, IOPSMENR_SPEC, bool, O>;
#[doc = "Field `IOPBSMEN` reader - I/O port B clock enable during Sleep mode"]
pub type IOPBSMEN_R = crate::BitReader<bool>;
#[doc = "Field `IOPBSMEN` writer - I/O port B clock enable during Sleep mode"]
pub type IOPBSMEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, IOPSMENR_SPEC, bool, O>;
#[doc = "Field `IOPCSMEN` reader - I/O port C clock enable during Sleep mode"]
pub type IOPCSMEN_R = crate::BitReader<bool>;
#[doc = "Field `IOPCSMEN` writer - I/O port C clock enable during Sleep mode"]
pub type IOPCSMEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, IOPSMENR_SPEC, bool, O>;
#[doc = "Field `IOPDSMEN` reader - I/O port D clock enable during Sleep mode"]
pub type IOPDSMEN_R = crate::BitReader<bool>;
#[doc = "Field `IOPDSMEN` writer - I/O port D clock enable during Sleep mode"]
pub type IOPDSMEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, IOPSMENR_SPEC, bool, O>;
#[doc = "Field `IOPFSMEN` reader - I/O port F clock enable during Sleep mode"]
pub type IOPFSMEN_R = crate::BitReader<bool>;
#[doc = "Field `IOPFSMEN` writer - I/O port F clock enable during Sleep mode"]
pub type IOPFSMEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, IOPSMENR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - I/O port A clock enable during Sleep mode"]
    #[inline(always)]
    pub fn iopasmen(&self) -> IOPASMEN_R {
        IOPASMEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - I/O port B clock enable during Sleep mode"]
    #[inline(always)]
    pub fn iopbsmen(&self) -> IOPBSMEN_R {
        IOPBSMEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - I/O port C clock enable during Sleep mode"]
    #[inline(always)]
    pub fn iopcsmen(&self) -> IOPCSMEN_R {
        IOPCSMEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - I/O port D clock enable during Sleep mode"]
    #[inline(always)]
    pub fn iopdsmen(&self) -> IOPDSMEN_R {
        IOPDSMEN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 5 - I/O port F clock enable during Sleep mode"]
    #[inline(always)]
    pub fn iopfsmen(&self) -> IOPFSMEN_R {
        IOPFSMEN_R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - I/O port A clock enable during Sleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn iopasmen(&mut self) -> IOPASMEN_W<0> {
        IOPASMEN_W::new(self)
    }
    #[doc = "Bit 1 - I/O port B clock enable during Sleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn iopbsmen(&mut self) -> IOPBSMEN_W<1> {
        IOPBSMEN_W::new(self)
    }
    #[doc = "Bit 2 - I/O port C clock enable during Sleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn iopcsmen(&mut self) -> IOPCSMEN_W<2> {
        IOPCSMEN_W::new(self)
    }
    #[doc = "Bit 3 - I/O port D clock enable during Sleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn iopdsmen(&mut self) -> IOPDSMEN_W<3> {
        IOPDSMEN_W::new(self)
    }
    #[doc = "Bit 5 - I/O port F clock enable during Sleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn iopfsmen(&mut self) -> IOPFSMEN_W<5> {
        IOPFSMEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "GPIO in Sleep mode clock enable register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [iopsmenr](index.html) module"]
pub struct IOPSMENR_SPEC;
impl crate::RegisterSpec for IOPSMENR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [iopsmenr::R](R) reader structure"]
impl crate::Readable for IOPSMENR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [iopsmenr::W](W) writer structure"]
impl crate::Writable for IOPSMENR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets IOPSMENR to value 0"]
impl crate::Resettable for IOPSMENR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
