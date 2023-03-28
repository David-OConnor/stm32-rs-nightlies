#[doc = "Register `C2AHB2ENR` reader"]
pub struct R(crate::R<C2AHB2ENR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<C2AHB2ENR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<C2AHB2ENR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<C2AHB2ENR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `C2AHB2ENR` writer"]
pub struct W(crate::W<C2AHB2ENR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<C2AHB2ENR_SPEC>;
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
impl From<crate::W<C2AHB2ENR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<C2AHB2ENR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `GPIOAEN` reader - CPU2 IO port A clock enable"]
pub type GPIOAEN_R = crate::BitReader<bool>;
#[doc = "Field `GPIOAEN` writer - CPU2 IO port A clock enable"]
pub type GPIOAEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, C2AHB2ENR_SPEC, bool, O>;
#[doc = "Field `GPIOBEN` reader - CPU2 IO port B clock enable"]
pub type GPIOBEN_R = crate::BitReader<bool>;
#[doc = "Field `GPIOBEN` writer - CPU2 IO port B clock enable"]
pub type GPIOBEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, C2AHB2ENR_SPEC, bool, O>;
#[doc = "Field `GPIOCEN` reader - CPU2 IO port C clock enable"]
pub type GPIOCEN_R = crate::BitReader<bool>;
#[doc = "Field `GPIOCEN` writer - CPU2 IO port C clock enable"]
pub type GPIOCEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, C2AHB2ENR_SPEC, bool, O>;
#[doc = "Field `GPIODEN` reader - CPU2 IO port D clock enable"]
pub type GPIODEN_R = crate::BitReader<bool>;
#[doc = "Field `GPIODEN` writer - CPU2 IO port D clock enable"]
pub type GPIODEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, C2AHB2ENR_SPEC, bool, O>;
#[doc = "Field `GPIOEEN` reader - CPU2 IO port E clock enable"]
pub type GPIOEEN_R = crate::BitReader<bool>;
#[doc = "Field `GPIOEEN` writer - CPU2 IO port E clock enable"]
pub type GPIOEEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, C2AHB2ENR_SPEC, bool, O>;
#[doc = "Field `GPIOHEN` reader - CPU2 IO port H clock enable"]
pub type GPIOHEN_R = crate::BitReader<bool>;
#[doc = "Field `GPIOHEN` writer - CPU2 IO port H clock enable"]
pub type GPIOHEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, C2AHB2ENR_SPEC, bool, O>;
#[doc = "Field `ADCEN` reader - CPU2 ADC clock enable"]
pub type ADCEN_R = crate::BitReader<bool>;
#[doc = "Field `ADCEN` writer - CPU2 ADC clock enable"]
pub type ADCEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, C2AHB2ENR_SPEC, bool, O>;
#[doc = "Field `AES1EN` reader - CPU2 AES1 accelerator clock enable"]
pub type AES1EN_R = crate::BitReader<bool>;
#[doc = "Field `AES1EN` writer - CPU2 AES1 accelerator clock enable"]
pub type AES1EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, C2AHB2ENR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - CPU2 IO port A clock enable"]
    #[inline(always)]
    pub fn gpioaen(&self) -> GPIOAEN_R {
        GPIOAEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - CPU2 IO port B clock enable"]
    #[inline(always)]
    pub fn gpioben(&self) -> GPIOBEN_R {
        GPIOBEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - CPU2 IO port C clock enable"]
    #[inline(always)]
    pub fn gpiocen(&self) -> GPIOCEN_R {
        GPIOCEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - CPU2 IO port D clock enable"]
    #[inline(always)]
    pub fn gpioden(&self) -> GPIODEN_R {
        GPIODEN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - CPU2 IO port E clock enable"]
    #[inline(always)]
    pub fn gpioeen(&self) -> GPIOEEN_R {
        GPIOEEN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 7 - CPU2 IO port H clock enable"]
    #[inline(always)]
    pub fn gpiohen(&self) -> GPIOHEN_R {
        GPIOHEN_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 13 - CPU2 ADC clock enable"]
    #[inline(always)]
    pub fn adcen(&self) -> ADCEN_R {
        ADCEN_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 16 - CPU2 AES1 accelerator clock enable"]
    #[inline(always)]
    pub fn aes1en(&self) -> AES1EN_R {
        AES1EN_R::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - CPU2 IO port A clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn gpioaen(&mut self) -> GPIOAEN_W<0> {
        GPIOAEN_W::new(self)
    }
    #[doc = "Bit 1 - CPU2 IO port B clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn gpioben(&mut self) -> GPIOBEN_W<1> {
        GPIOBEN_W::new(self)
    }
    #[doc = "Bit 2 - CPU2 IO port C clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn gpiocen(&mut self) -> GPIOCEN_W<2> {
        GPIOCEN_W::new(self)
    }
    #[doc = "Bit 3 - CPU2 IO port D clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn gpioden(&mut self) -> GPIODEN_W<3> {
        GPIODEN_W::new(self)
    }
    #[doc = "Bit 4 - CPU2 IO port E clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn gpioeen(&mut self) -> GPIOEEN_W<4> {
        GPIOEEN_W::new(self)
    }
    #[doc = "Bit 7 - CPU2 IO port H clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn gpiohen(&mut self) -> GPIOHEN_W<7> {
        GPIOHEN_W::new(self)
    }
    #[doc = "Bit 13 - CPU2 ADC clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn adcen(&mut self) -> ADCEN_W<13> {
        ADCEN_W::new(self)
    }
    #[doc = "Bit 16 - CPU2 AES1 accelerator clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn aes1en(&mut self) -> AES1EN_W<16> {
        AES1EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CPU2 AHB2 peripheral clock enable register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [c2ahb2enr](index.html) module"]
pub struct C2AHB2ENR_SPEC;
impl crate::RegisterSpec for C2AHB2ENR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [c2ahb2enr::R](R) reader structure"]
impl crate::Readable for C2AHB2ENR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [c2ahb2enr::W](W) writer structure"]
impl crate::Writable for C2AHB2ENR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets C2AHB2ENR to value 0"]
impl crate::Resettable for C2AHB2ENR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
