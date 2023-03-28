#[doc = "Register `C2IMR1` reader"]
pub struct R(crate::R<C2IMR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<C2IMR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<C2IMR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<C2IMR1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `C2IMR1` writer"]
pub struct W(crate::W<C2IMR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<C2IMR1_SPEC>;
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
impl From<crate::W<C2IMR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<C2IMR1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RTCSTAMP` reader - Peripheral RTCSTAMP interrupt mask to CPU2"]
pub type RTCSTAMP_R = crate::BitReader<bool>;
#[doc = "Field `RTCSTAMP` writer - Peripheral RTCSTAMP interrupt mask to CPU2"]
pub type RTCSTAMP_W<'a, const O: u8> = crate::BitWriter<'a, u32, C2IMR1_SPEC, bool, O>;
#[doc = "Field `RTCWKUP` reader - Peripheral RTCWKUP interrupt mask to CPU2"]
pub type RTCWKUP_R = crate::BitReader<bool>;
#[doc = "Field `RTCWKUP` writer - Peripheral RTCWKUP interrupt mask to CPU2"]
pub type RTCWKUP_W<'a, const O: u8> = crate::BitWriter<'a, u32, C2IMR1_SPEC, bool, O>;
#[doc = "Field `RTCALARM` reader - Peripheral RTCALARM interrupt mask to CPU2"]
pub type RTCALARM_R = crate::BitReader<bool>;
#[doc = "Field `RTCALARM` writer - Peripheral RTCALARM interrupt mask to CPU2"]
pub type RTCALARM_W<'a, const O: u8> = crate::BitWriter<'a, u32, C2IMR1_SPEC, bool, O>;
#[doc = "Field `RCC` reader - Peripheral RCC interrupt mask to CPU2"]
pub type RCC_R = crate::BitReader<bool>;
#[doc = "Field `RCC` writer - Peripheral RCC interrupt mask to CPU2"]
pub type RCC_W<'a, const O: u8> = crate::BitWriter<'a, u32, C2IMR1_SPEC, bool, O>;
#[doc = "Field `FLASH` reader - Peripheral FLASH interrupt mask to CPU2"]
pub type FLASH_R = crate::BitReader<bool>;
#[doc = "Field `FLASH` writer - Peripheral FLASH interrupt mask to CPU2"]
pub type FLASH_W<'a, const O: u8> = crate::BitWriter<'a, u32, C2IMR1_SPEC, bool, O>;
#[doc = "Field `PKA` reader - Peripheral PKA interrupt mask to CPU2"]
pub type PKA_R = crate::BitReader<bool>;
#[doc = "Field `PKA` writer - Peripheral PKA interrupt mask to CPU2"]
pub type PKA_W<'a, const O: u8> = crate::BitWriter<'a, u32, C2IMR1_SPEC, bool, O>;
#[doc = "Field `RNG` reader - Peripheral RNG interrupt mask to CPU2"]
pub type RNG_R = crate::BitReader<bool>;
#[doc = "Field `RNG` writer - Peripheral RNG interrupt mask to CPU2"]
pub type RNG_W<'a, const O: u8> = crate::BitWriter<'a, u32, C2IMR1_SPEC, bool, O>;
#[doc = "Field `AES1` reader - Peripheral AES1 interrupt mask to CPU2"]
pub type AES1_R = crate::BitReader<bool>;
#[doc = "Field `AES1` writer - Peripheral AES1 interrupt mask to CPU2"]
pub type AES1_W<'a, const O: u8> = crate::BitWriter<'a, u32, C2IMR1_SPEC, bool, O>;
#[doc = "Field `COMP` reader - Peripheral COMP interrupt mask to CPU2"]
pub type COMP_R = crate::BitReader<bool>;
#[doc = "Field `COMP` writer - Peripheral COMP interrupt mask to CPU2"]
pub type COMP_W<'a, const O: u8> = crate::BitWriter<'a, u32, C2IMR1_SPEC, bool, O>;
#[doc = "Field `ADC` reader - Peripheral ADC interrupt mask to CPU2"]
pub type ADC_R = crate::BitReader<bool>;
#[doc = "Field `ADC` writer - Peripheral ADC interrupt mask to CPU2"]
pub type ADC_W<'a, const O: u8> = crate::BitWriter<'a, u32, C2IMR1_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Peripheral RTCSTAMP interrupt mask to CPU2"]
    #[inline(always)]
    pub fn rtcstamp(&self) -> RTCSTAMP_R {
        RTCSTAMP_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 3 - Peripheral RTCWKUP interrupt mask to CPU2"]
    #[inline(always)]
    pub fn rtcwkup(&self) -> RTCWKUP_R {
        RTCWKUP_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Peripheral RTCALARM interrupt mask to CPU2"]
    #[inline(always)]
    pub fn rtcalarm(&self) -> RTCALARM_R {
        RTCALARM_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Peripheral RCC interrupt mask to CPU2"]
    #[inline(always)]
    pub fn rcc(&self) -> RCC_R {
        RCC_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Peripheral FLASH interrupt mask to CPU2"]
    #[inline(always)]
    pub fn flash(&self) -> FLASH_R {
        FLASH_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 8 - Peripheral PKA interrupt mask to CPU2"]
    #[inline(always)]
    pub fn pka(&self) -> PKA_R {
        PKA_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Peripheral RNG interrupt mask to CPU2"]
    #[inline(always)]
    pub fn rng(&self) -> RNG_R {
        RNG_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Peripheral AES1 interrupt mask to CPU2"]
    #[inline(always)]
    pub fn aes1(&self) -> AES1_R {
        AES1_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Peripheral COMP interrupt mask to CPU2"]
    #[inline(always)]
    pub fn comp(&self) -> COMP_R {
        COMP_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Peripheral ADC interrupt mask to CPU2"]
    #[inline(always)]
    pub fn adc(&self) -> ADC_R {
        ADC_R::new(((self.bits >> 12) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Peripheral RTCSTAMP interrupt mask to CPU2"]
    #[inline(always)]
    #[must_use]
    pub fn rtcstamp(&mut self) -> RTCSTAMP_W<0> {
        RTCSTAMP_W::new(self)
    }
    #[doc = "Bit 3 - Peripheral RTCWKUP interrupt mask to CPU2"]
    #[inline(always)]
    #[must_use]
    pub fn rtcwkup(&mut self) -> RTCWKUP_W<3> {
        RTCWKUP_W::new(self)
    }
    #[doc = "Bit 4 - Peripheral RTCALARM interrupt mask to CPU2"]
    #[inline(always)]
    #[must_use]
    pub fn rtcalarm(&mut self) -> RTCALARM_W<4> {
        RTCALARM_W::new(self)
    }
    #[doc = "Bit 5 - Peripheral RCC interrupt mask to CPU2"]
    #[inline(always)]
    #[must_use]
    pub fn rcc(&mut self) -> RCC_W<5> {
        RCC_W::new(self)
    }
    #[doc = "Bit 6 - Peripheral FLASH interrupt mask to CPU2"]
    #[inline(always)]
    #[must_use]
    pub fn flash(&mut self) -> FLASH_W<6> {
        FLASH_W::new(self)
    }
    #[doc = "Bit 8 - Peripheral PKA interrupt mask to CPU2"]
    #[inline(always)]
    #[must_use]
    pub fn pka(&mut self) -> PKA_W<8> {
        PKA_W::new(self)
    }
    #[doc = "Bit 9 - Peripheral RNG interrupt mask to CPU2"]
    #[inline(always)]
    #[must_use]
    pub fn rng(&mut self) -> RNG_W<9> {
        RNG_W::new(self)
    }
    #[doc = "Bit 10 - Peripheral AES1 interrupt mask to CPU2"]
    #[inline(always)]
    #[must_use]
    pub fn aes1(&mut self) -> AES1_W<10> {
        AES1_W::new(self)
    }
    #[doc = "Bit 11 - Peripheral COMP interrupt mask to CPU2"]
    #[inline(always)]
    #[must_use]
    pub fn comp(&mut self) -> COMP_W<11> {
        COMP_W::new(self)
    }
    #[doc = "Bit 12 - Peripheral ADC interrupt mask to CPU2"]
    #[inline(always)]
    #[must_use]
    pub fn adc(&mut self) -> ADC_W<12> {
        ADC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CPU2 interrupt mask register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [c2imr1](index.html) module"]
pub struct C2IMR1_SPEC;
impl crate::RegisterSpec for C2IMR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [c2imr1::R](R) reader structure"]
impl crate::Readable for C2IMR1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [c2imr1::W](W) writer structure"]
impl crate::Writable for C2IMR1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets C2IMR1 to value 0"]
impl crate::Resettable for C2IMR1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
