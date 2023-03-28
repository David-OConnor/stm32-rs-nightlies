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
#[doc = "Field `GPIOAEN` reader - I/O port A clock enable This bit is set and cleared by software."]
pub type GPIOAEN_R = crate::BitReader<bool>;
#[doc = "Field `GPIOAEN` writer - I/O port A clock enable This bit is set and cleared by software."]
pub type GPIOAEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, IOPENR_SPEC, bool, O>;
#[doc = "Field `GPIOBEN` reader - I/O port B clock enable This bit is set and cleared by software."]
pub type GPIOBEN_R = crate::BitReader<bool>;
#[doc = "Field `GPIOBEN` writer - I/O port B clock enable This bit is set and cleared by software."]
pub type GPIOBEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, IOPENR_SPEC, bool, O>;
#[doc = "Field `GPIOCEN` reader - I/O port C clock enable This bit is set and cleared by software."]
pub type GPIOCEN_R = crate::BitReader<bool>;
#[doc = "Field `GPIOCEN` writer - I/O port C clock enable This bit is set and cleared by software."]
pub type GPIOCEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, IOPENR_SPEC, bool, O>;
#[doc = "Field `GPIODEN` reader - I/O port D clock enable This bit is set and cleared by software."]
pub type GPIODEN_R = crate::BitReader<bool>;
#[doc = "Field `GPIODEN` writer - I/O port D clock enable This bit is set and cleared by software."]
pub type GPIODEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, IOPENR_SPEC, bool, O>;
#[doc = "Field `GPIOFEN` reader - I/O port F clock enable This bit is set and cleared by software."]
pub type GPIOFEN_R = crate::BitReader<bool>;
#[doc = "Field `GPIOFEN` writer - I/O port F clock enable This bit is set and cleared by software."]
pub type GPIOFEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, IOPENR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - I/O port A clock enable This bit is set and cleared by software."]
    #[inline(always)]
    pub fn gpioaen(&self) -> GPIOAEN_R {
        GPIOAEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - I/O port B clock enable This bit is set and cleared by software."]
    #[inline(always)]
    pub fn gpioben(&self) -> GPIOBEN_R {
        GPIOBEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - I/O port C clock enable This bit is set and cleared by software."]
    #[inline(always)]
    pub fn gpiocen(&self) -> GPIOCEN_R {
        GPIOCEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - I/O port D clock enable This bit is set and cleared by software."]
    #[inline(always)]
    pub fn gpioden(&self) -> GPIODEN_R {
        GPIODEN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 5 - I/O port F clock enable This bit is set and cleared by software."]
    #[inline(always)]
    pub fn gpiofen(&self) -> GPIOFEN_R {
        GPIOFEN_R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - I/O port A clock enable This bit is set and cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn gpioaen(&mut self) -> GPIOAEN_W<0> {
        GPIOAEN_W::new(self)
    }
    #[doc = "Bit 1 - I/O port B clock enable This bit is set and cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn gpioben(&mut self) -> GPIOBEN_W<1> {
        GPIOBEN_W::new(self)
    }
    #[doc = "Bit 2 - I/O port C clock enable This bit is set and cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn gpiocen(&mut self) -> GPIOCEN_W<2> {
        GPIOCEN_W::new(self)
    }
    #[doc = "Bit 3 - I/O port D clock enable This bit is set and cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn gpioden(&mut self) -> GPIODEN_W<3> {
        GPIODEN_W::new(self)
    }
    #[doc = "Bit 5 - I/O port F clock enable This bit is set and cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn gpiofen(&mut self) -> GPIOFEN_W<5> {
        GPIOFEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RCC I/O port clock enable register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [iopenr](index.html) module"]
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
