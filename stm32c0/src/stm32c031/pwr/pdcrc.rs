#[doc = "Register `PDCRC` reader"]
pub struct R(crate::R<PDCRC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PDCRC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PDCRC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PDCRC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PDCRC` writer"]
pub struct W(crate::W<PDCRC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PDCRC_SPEC>;
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
impl From<crate::W<PDCRC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PDCRC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PD6` reader - Port C pull-down bit i (i = 15, 14, 13, 7, 6) Setting PDi bit while the APC bit of the PWR_CR3 register is set activates a pull-down device on the PC\\[i\\]
I/O. On STM32C011xx, only PD15 and PD14 are available."]
pub type PD6_R = crate::BitReader<bool>;
#[doc = "Field `PD6` writer - Port C pull-down bit i (i = 15, 14, 13, 7, 6) Setting PDi bit while the APC bit of the PWR_CR3 register is set activates a pull-down device on the PC\\[i\\]
I/O. On STM32C011xx, only PD15 and PD14 are available."]
pub type PD6_W<'a, const O: u8> = crate::BitWriter<'a, u32, PDCRC_SPEC, bool, O>;
#[doc = "Field `PD7` reader - Port C pull-down bit i (i = 15, 14, 13, 7, 6) Setting PDi bit while the APC bit of the PWR_CR3 register is set activates a pull-down device on the PC\\[i\\]
I/O. On STM32C011xx, only PD15 and PD14 are available."]
pub type PD7_R = crate::BitReader<bool>;
#[doc = "Field `PD7` writer - Port C pull-down bit i (i = 15, 14, 13, 7, 6) Setting PDi bit while the APC bit of the PWR_CR3 register is set activates a pull-down device on the PC\\[i\\]
I/O. On STM32C011xx, only PD15 and PD14 are available."]
pub type PD7_W<'a, const O: u8> = crate::BitWriter<'a, u32, PDCRC_SPEC, bool, O>;
#[doc = "Field `PD13` reader - Port C pull-down bit i (i = 15, 14, 13, 7, 6) Setting PDi bit while the APC bit of the PWR_CR3 register is set activates a pull-down device on the PC\\[i\\]
I/O. On STM32C011xx, only PD15 and PD14 are available."]
pub type PD13_R = crate::BitReader<bool>;
#[doc = "Field `PD13` writer - Port C pull-down bit i (i = 15, 14, 13, 7, 6) Setting PDi bit while the APC bit of the PWR_CR3 register is set activates a pull-down device on the PC\\[i\\]
I/O. On STM32C011xx, only PD15 and PD14 are available."]
pub type PD13_W<'a, const O: u8> = crate::BitWriter<'a, u32, PDCRC_SPEC, bool, O>;
#[doc = "Field `PD14` reader - Port C pull-down bit i (i = 15, 14, 13, 7, 6) Setting PDi bit while the APC bit of the PWR_CR3 register is set activates a pull-down device on the PC\\[i\\]
I/O. On STM32C011xx, only PD15 and PD14 are available."]
pub type PD14_R = crate::BitReader<bool>;
#[doc = "Field `PD14` writer - Port C pull-down bit i (i = 15, 14, 13, 7, 6) Setting PDi bit while the APC bit of the PWR_CR3 register is set activates a pull-down device on the PC\\[i\\]
I/O. On STM32C011xx, only PD15 and PD14 are available."]
pub type PD14_W<'a, const O: u8> = crate::BitWriter<'a, u32, PDCRC_SPEC, bool, O>;
#[doc = "Field `PD15` reader - Port C pull-down bit i (i = 15, 14, 13, 7, 6) Setting PDi bit while the APC bit of the PWR_CR3 register is set activates a pull-down device on the PC\\[i\\]
I/O. On STM32C011xx, only PD15 and PD14 are available."]
pub type PD15_R = crate::BitReader<bool>;
#[doc = "Field `PD15` writer - Port C pull-down bit i (i = 15, 14, 13, 7, 6) Setting PDi bit while the APC bit of the PWR_CR3 register is set activates a pull-down device on the PC\\[i\\]
I/O. On STM32C011xx, only PD15 and PD14 are available."]
pub type PD15_W<'a, const O: u8> = crate::BitWriter<'a, u32, PDCRC_SPEC, bool, O>;
impl R {
    #[doc = "Bit 6 - Port C pull-down bit i (i = 15, 14, 13, 7, 6) Setting PDi bit while the APC bit of the PWR_CR3 register is set activates a pull-down device on the PC\\[i\\]
I/O. On STM32C011xx, only PD15 and PD14 are available."]
    #[inline(always)]
    pub fn pd6(&self) -> PD6_R {
        PD6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Port C pull-down bit i (i = 15, 14, 13, 7, 6) Setting PDi bit while the APC bit of the PWR_CR3 register is set activates a pull-down device on the PC\\[i\\]
I/O. On STM32C011xx, only PD15 and PD14 are available."]
    #[inline(always)]
    pub fn pd7(&self) -> PD7_R {
        PD7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 13 - Port C pull-down bit i (i = 15, 14, 13, 7, 6) Setting PDi bit while the APC bit of the PWR_CR3 register is set activates a pull-down device on the PC\\[i\\]
I/O. On STM32C011xx, only PD15 and PD14 are available."]
    #[inline(always)]
    pub fn pd13(&self) -> PD13_R {
        PD13_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Port C pull-down bit i (i = 15, 14, 13, 7, 6) Setting PDi bit while the APC bit of the PWR_CR3 register is set activates a pull-down device on the PC\\[i\\]
I/O. On STM32C011xx, only PD15 and PD14 are available."]
    #[inline(always)]
    pub fn pd14(&self) -> PD14_R {
        PD14_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Port C pull-down bit i (i = 15, 14, 13, 7, 6) Setting PDi bit while the APC bit of the PWR_CR3 register is set activates a pull-down device on the PC\\[i\\]
I/O. On STM32C011xx, only PD15 and PD14 are available."]
    #[inline(always)]
    pub fn pd15(&self) -> PD15_R {
        PD15_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 6 - Port C pull-down bit i (i = 15, 14, 13, 7, 6) Setting PDi bit while the APC bit of the PWR_CR3 register is set activates a pull-down device on the PC\\[i\\]
I/O. On STM32C011xx, only PD15 and PD14 are available."]
    #[inline(always)]
    #[must_use]
    pub fn pd6(&mut self) -> PD6_W<6> {
        PD6_W::new(self)
    }
    #[doc = "Bit 7 - Port C pull-down bit i (i = 15, 14, 13, 7, 6) Setting PDi bit while the APC bit of the PWR_CR3 register is set activates a pull-down device on the PC\\[i\\]
I/O. On STM32C011xx, only PD15 and PD14 are available."]
    #[inline(always)]
    #[must_use]
    pub fn pd7(&mut self) -> PD7_W<7> {
        PD7_W::new(self)
    }
    #[doc = "Bit 13 - Port C pull-down bit i (i = 15, 14, 13, 7, 6) Setting PDi bit while the APC bit of the PWR_CR3 register is set activates a pull-down device on the PC\\[i\\]
I/O. On STM32C011xx, only PD15 and PD14 are available."]
    #[inline(always)]
    #[must_use]
    pub fn pd13(&mut self) -> PD13_W<13> {
        PD13_W::new(self)
    }
    #[doc = "Bit 14 - Port C pull-down bit i (i = 15, 14, 13, 7, 6) Setting PDi bit while the APC bit of the PWR_CR3 register is set activates a pull-down device on the PC\\[i\\]
I/O. On STM32C011xx, only PD15 and PD14 are available."]
    #[inline(always)]
    #[must_use]
    pub fn pd14(&mut self) -> PD14_W<14> {
        PD14_W::new(self)
    }
    #[doc = "Bit 15 - Port C pull-down bit i (i = 15, 14, 13, 7, 6) Setting PDi bit while the APC bit of the PWR_CR3 register is set activates a pull-down device on the PC\\[i\\]
I/O. On STM32C011xx, only PD15 and PD14 are available."]
    #[inline(always)]
    #[must_use]
    pub fn pd15(&mut self) -> PD15_W<15> {
        PD15_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PWR Port C pull-down control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pdcrc](index.html) module"]
pub struct PDCRC_SPEC;
impl crate::RegisterSpec for PDCRC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pdcrc::R](R) reader structure"]
impl crate::Readable for PDCRC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pdcrc::W](W) writer structure"]
impl crate::Writable for PDCRC_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PDCRC to value 0"]
impl crate::Resettable for PDCRC_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
