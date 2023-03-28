#[doc = "Register `AHB2SMENR` reader"]
pub struct R(crate::R<AHB2SMENR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<AHB2SMENR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<AHB2SMENR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<AHB2SMENR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `AHB2SMENR` writer"]
pub struct W(crate::W<AHB2SMENR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<AHB2SMENR_SPEC>;
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
impl From<crate::W<AHB2SMENR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<AHB2SMENR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `GPIOASMEN` reader - IO port A clocks enable during Sleep and Stop modes"]
pub type GPIOASMEN_R = crate::BitReader<bool>;
#[doc = "Field `GPIOASMEN` writer - IO port A clocks enable during Sleep and Stop modes"]
pub type GPIOASMEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, AHB2SMENR_SPEC, bool, O>;
#[doc = "Field `GPIOBSMEN` reader - IO port B clocks enable during Sleep and Stop modes"]
pub type GPIOBSMEN_R = crate::BitReader<bool>;
#[doc = "Field `GPIOBSMEN` writer - IO port B clocks enable during Sleep and Stop modes"]
pub type GPIOBSMEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, AHB2SMENR_SPEC, bool, O>;
#[doc = "Field `GPIOCSMEN` reader - IO port C clocks enable during Sleep and Stop modes"]
pub type GPIOCSMEN_R = crate::BitReader<bool>;
#[doc = "Field `GPIOCSMEN` writer - IO port C clocks enable during Sleep and Stop modes"]
pub type GPIOCSMEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, AHB2SMENR_SPEC, bool, O>;
#[doc = "Field `GPIODSMEN` reader - IO port D clocks enable during Sleep and Stop modes"]
pub type GPIODSMEN_R = crate::BitReader<bool>;
#[doc = "Field `GPIODSMEN` writer - IO port D clocks enable during Sleep and Stop modes"]
pub type GPIODSMEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, AHB2SMENR_SPEC, bool, O>;
#[doc = "Field `GPIOESMEN` reader - IO port E clocks enable during Sleep and Stop modes"]
pub type GPIOESMEN_R = crate::BitReader<bool>;
#[doc = "Field `GPIOESMEN` writer - IO port E clocks enable during Sleep and Stop modes"]
pub type GPIOESMEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, AHB2SMENR_SPEC, bool, O>;
#[doc = "Field `GPIOFSMEN` reader - IO port F clocks enable during Sleep and Stop modes"]
pub type GPIOFSMEN_R = crate::BitReader<bool>;
#[doc = "Field `GPIOFSMEN` writer - IO port F clocks enable during Sleep and Stop modes"]
pub type GPIOFSMEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, AHB2SMENR_SPEC, bool, O>;
#[doc = "Field `GPIOGSMEN` reader - IO port G clocks enable during Sleep and Stop modes"]
pub type GPIOGSMEN_R = crate::BitReader<bool>;
#[doc = "Field `GPIOGSMEN` writer - IO port G clocks enable during Sleep and Stop modes"]
pub type GPIOGSMEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, AHB2SMENR_SPEC, bool, O>;
#[doc = "Field `CCMSRAMSMEN` reader - CCM SRAM interface clocks enable during Sleep and Stop modes"]
pub type CCMSRAMSMEN_R = crate::BitReader<bool>;
#[doc = "Field `CCMSRAMSMEN` writer - CCM SRAM interface clocks enable during Sleep and Stop modes"]
pub type CCMSRAMSMEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, AHB2SMENR_SPEC, bool, O>;
#[doc = "Field `SRAM2SMEN` reader - SRAM2 interface clocks enable during Sleep and Stop modes"]
pub type SRAM2SMEN_R = crate::BitReader<bool>;
#[doc = "Field `SRAM2SMEN` writer - SRAM2 interface clocks enable during Sleep and Stop modes"]
pub type SRAM2SMEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, AHB2SMENR_SPEC, bool, O>;
#[doc = "Field `ADC12SMEN` reader - ADC clocks enable during Sleep and Stop modes"]
pub type ADC12SMEN_R = crate::BitReader<bool>;
#[doc = "Field `ADC12SMEN` writer - ADC clocks enable during Sleep and Stop modes"]
pub type ADC12SMEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, AHB2SMENR_SPEC, bool, O>;
#[doc = "Field `ADC345SMEN` reader - DCMI clock enable during Sleep and Stop modes"]
pub type ADC345SMEN_R = crate::BitReader<bool>;
#[doc = "Field `ADC345SMEN` writer - DCMI clock enable during Sleep and Stop modes"]
pub type ADC345SMEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, AHB2SMENR_SPEC, bool, O>;
#[doc = "Field `DAC1SMEN` reader - AES accelerator clocks enable during Sleep and Stop modes"]
pub type DAC1SMEN_R = crate::BitReader<bool>;
#[doc = "Field `DAC1SMEN` writer - AES accelerator clocks enable during Sleep and Stop modes"]
pub type DAC1SMEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, AHB2SMENR_SPEC, bool, O>;
#[doc = "Field `DAC2SMEN` reader - HASH clock enable during Sleep and Stop modes"]
pub type DAC2SMEN_R = crate::BitReader<bool>;
#[doc = "Field `DAC2SMEN` writer - HASH clock enable during Sleep and Stop modes"]
pub type DAC2SMEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, AHB2SMENR_SPEC, bool, O>;
#[doc = "Field `DAC3SMEN` reader - DAC3 clock enable during sleep mode"]
pub type DAC3SMEN_R = crate::BitReader<bool>;
#[doc = "Field `DAC3SMEN` writer - DAC3 clock enable during sleep mode"]
pub type DAC3SMEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, AHB2SMENR_SPEC, bool, O>;
#[doc = "Field `DAC4SMEN` reader - DAC4 clock enable during sleep mode"]
pub type DAC4SMEN_R = crate::BitReader<bool>;
#[doc = "Field `DAC4SMEN` writer - DAC4 clock enable during sleep mode"]
pub type DAC4SMEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, AHB2SMENR_SPEC, bool, O>;
#[doc = "Field `AESMEN` reader - Cryptography clock enable during sleep mode"]
pub type AESMEN_R = crate::BitReader<bool>;
#[doc = "Field `AESMEN` writer - Cryptography clock enable during sleep mode"]
pub type AESMEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, AHB2SMENR_SPEC, bool, O>;
#[doc = "Field `RNGSMEN` reader - Random Number Generator clock enable during sleep mode"]
pub type RNGSMEN_R = crate::BitReader<bool>;
#[doc = "Field `RNGSMEN` writer - Random Number Generator clock enable during sleep mode"]
pub type RNGSMEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, AHB2SMENR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - IO port A clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    pub fn gpioasmen(&self) -> GPIOASMEN_R {
        GPIOASMEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - IO port B clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    pub fn gpiobsmen(&self) -> GPIOBSMEN_R {
        GPIOBSMEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - IO port C clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    pub fn gpiocsmen(&self) -> GPIOCSMEN_R {
        GPIOCSMEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - IO port D clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    pub fn gpiodsmen(&self) -> GPIODSMEN_R {
        GPIODSMEN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - IO port E clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    pub fn gpioesmen(&self) -> GPIOESMEN_R {
        GPIOESMEN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - IO port F clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    pub fn gpiofsmen(&self) -> GPIOFSMEN_R {
        GPIOFSMEN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - IO port G clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    pub fn gpiogsmen(&self) -> GPIOGSMEN_R {
        GPIOGSMEN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 9 - CCM SRAM interface clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    pub fn ccmsramsmen(&self) -> CCMSRAMSMEN_R {
        CCMSRAMSMEN_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - SRAM2 interface clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    pub fn sram2smen(&self) -> SRAM2SMEN_R {
        SRAM2SMEN_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 13 - ADC clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    pub fn adc12smen(&self) -> ADC12SMEN_R {
        ADC12SMEN_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - DCMI clock enable during Sleep and Stop modes"]
    #[inline(always)]
    pub fn adc345smen(&self) -> ADC345SMEN_R {
        ADC345SMEN_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 16 - AES accelerator clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    pub fn dac1smen(&self) -> DAC1SMEN_R {
        DAC1SMEN_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - HASH clock enable during Sleep and Stop modes"]
    #[inline(always)]
    pub fn dac2smen(&self) -> DAC2SMEN_R {
        DAC2SMEN_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - DAC3 clock enable during sleep mode"]
    #[inline(always)]
    pub fn dac3smen(&self) -> DAC3SMEN_R {
        DAC3SMEN_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - DAC4 clock enable during sleep mode"]
    #[inline(always)]
    pub fn dac4smen(&self) -> DAC4SMEN_R {
        DAC4SMEN_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 24 - Cryptography clock enable during sleep mode"]
    #[inline(always)]
    pub fn aesmen(&self) -> AESMEN_R {
        AESMEN_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 26 - Random Number Generator clock enable during sleep mode"]
    #[inline(always)]
    pub fn rngsmen(&self) -> RNGSMEN_R {
        RNGSMEN_R::new(((self.bits >> 26) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - IO port A clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    #[must_use]
    pub fn gpioasmen(&mut self) -> GPIOASMEN_W<0> {
        GPIOASMEN_W::new(self)
    }
    #[doc = "Bit 1 - IO port B clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    #[must_use]
    pub fn gpiobsmen(&mut self) -> GPIOBSMEN_W<1> {
        GPIOBSMEN_W::new(self)
    }
    #[doc = "Bit 2 - IO port C clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    #[must_use]
    pub fn gpiocsmen(&mut self) -> GPIOCSMEN_W<2> {
        GPIOCSMEN_W::new(self)
    }
    #[doc = "Bit 3 - IO port D clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    #[must_use]
    pub fn gpiodsmen(&mut self) -> GPIODSMEN_W<3> {
        GPIODSMEN_W::new(self)
    }
    #[doc = "Bit 4 - IO port E clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    #[must_use]
    pub fn gpioesmen(&mut self) -> GPIOESMEN_W<4> {
        GPIOESMEN_W::new(self)
    }
    #[doc = "Bit 5 - IO port F clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    #[must_use]
    pub fn gpiofsmen(&mut self) -> GPIOFSMEN_W<5> {
        GPIOFSMEN_W::new(self)
    }
    #[doc = "Bit 6 - IO port G clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    #[must_use]
    pub fn gpiogsmen(&mut self) -> GPIOGSMEN_W<6> {
        GPIOGSMEN_W::new(self)
    }
    #[doc = "Bit 9 - CCM SRAM interface clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    #[must_use]
    pub fn ccmsramsmen(&mut self) -> CCMSRAMSMEN_W<9> {
        CCMSRAMSMEN_W::new(self)
    }
    #[doc = "Bit 10 - SRAM2 interface clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    #[must_use]
    pub fn sram2smen(&mut self) -> SRAM2SMEN_W<10> {
        SRAM2SMEN_W::new(self)
    }
    #[doc = "Bit 13 - ADC clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    #[must_use]
    pub fn adc12smen(&mut self) -> ADC12SMEN_W<13> {
        ADC12SMEN_W::new(self)
    }
    #[doc = "Bit 14 - DCMI clock enable during Sleep and Stop modes"]
    #[inline(always)]
    #[must_use]
    pub fn adc345smen(&mut self) -> ADC345SMEN_W<14> {
        ADC345SMEN_W::new(self)
    }
    #[doc = "Bit 16 - AES accelerator clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    #[must_use]
    pub fn dac1smen(&mut self) -> DAC1SMEN_W<16> {
        DAC1SMEN_W::new(self)
    }
    #[doc = "Bit 17 - HASH clock enable during Sleep and Stop modes"]
    #[inline(always)]
    #[must_use]
    pub fn dac2smen(&mut self) -> DAC2SMEN_W<17> {
        DAC2SMEN_W::new(self)
    }
    #[doc = "Bit 18 - DAC3 clock enable during sleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn dac3smen(&mut self) -> DAC3SMEN_W<18> {
        DAC3SMEN_W::new(self)
    }
    #[doc = "Bit 19 - DAC4 clock enable during sleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn dac4smen(&mut self) -> DAC4SMEN_W<19> {
        DAC4SMEN_W::new(self)
    }
    #[doc = "Bit 24 - Cryptography clock enable during sleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn aesmen(&mut self) -> AESMEN_W<24> {
        AESMEN_W::new(self)
    }
    #[doc = "Bit 26 - Random Number Generator clock enable during sleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn rngsmen(&mut self) -> RNGSMEN_W<26> {
        RNGSMEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "AHB2 peripheral clocks enable in Sleep and Stop modes register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ahb2smenr](index.html) module"]
pub struct AHB2SMENR_SPEC;
impl crate::RegisterSpec for AHB2SMENR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ahb2smenr::R](R) reader structure"]
impl crate::Readable for AHB2SMENR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ahb2smenr::W](W) writer structure"]
impl crate::Writable for AHB2SMENR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets AHB2SMENR to value 0x050f_667f"]
impl crate::Resettable for AHB2SMENR_SPEC {
    const RESET_VALUE: Self::Ux = 0x050f_667f;
}
