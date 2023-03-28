#[doc = "Register `PDCRB` reader"]
pub struct R(crate::R<PDCRB_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PDCRB_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PDCRB_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PDCRB_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PDCRB` writer"]
pub struct W(crate::W<PDCRB_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PDCRB_SPEC>;
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
impl From<crate::W<PDCRB_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PDCRB_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PD6` reader - Port B pull-down bit i (i = 15 to 0) Setting PDi bit while the APC bit of the PWR_CR3 register is set activates a pull-down device on the PB\\[i\\]
I/O. On STM32C011xx, only PD7 and PD6 are available"]
pub type PD6_R = crate::BitReader<bool>;
#[doc = "Field `PD6` writer - Port B pull-down bit i (i = 15 to 0) Setting PDi bit while the APC bit of the PWR_CR3 register is set activates a pull-down device on the PB\\[i\\]
I/O. On STM32C011xx, only PD7 and PD6 are available"]
pub type PD6_W<'a, const O: u8> = crate::BitWriter<'a, u32, PDCRB_SPEC, bool, O>;
#[doc = "Field `PD7` reader - Port B pull-down bit i (i = 15 to 0) Setting PDi bit while the APC bit of the PWR_CR3 register is set activates a pull-down device on the PB\\[i\\]
I/O. On STM32C011xx, only PD7 and PD6 are available"]
pub type PD7_R = crate::BitReader<bool>;
#[doc = "Field `PD7` writer - Port B pull-down bit i (i = 15 to 0) Setting PDi bit while the APC bit of the PWR_CR3 register is set activates a pull-down device on the PB\\[i\\]
I/O. On STM32C011xx, only PD7 and PD6 are available"]
pub type PD7_W<'a, const O: u8> = crate::BitWriter<'a, u32, PDCRB_SPEC, bool, O>;
impl R {
    #[doc = "Bit 6 - Port B pull-down bit i (i = 15 to 0) Setting PDi bit while the APC bit of the PWR_CR3 register is set activates a pull-down device on the PB\\[i\\]
I/O. On STM32C011xx, only PD7 and PD6 are available"]
    #[inline(always)]
    pub fn pd6(&self) -> PD6_R {
        PD6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Port B pull-down bit i (i = 15 to 0) Setting PDi bit while the APC bit of the PWR_CR3 register is set activates a pull-down device on the PB\\[i\\]
I/O. On STM32C011xx, only PD7 and PD6 are available"]
    #[inline(always)]
    pub fn pd7(&self) -> PD7_R {
        PD7_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 6 - Port B pull-down bit i (i = 15 to 0) Setting PDi bit while the APC bit of the PWR_CR3 register is set activates a pull-down device on the PB\\[i\\]
I/O. On STM32C011xx, only PD7 and PD6 are available"]
    #[inline(always)]
    #[must_use]
    pub fn pd6(&mut self) -> PD6_W<6> {
        PD6_W::new(self)
    }
    #[doc = "Bit 7 - Port B pull-down bit i (i = 15 to 0) Setting PDi bit while the APC bit of the PWR_CR3 register is set activates a pull-down device on the PB\\[i\\]
I/O. On STM32C011xx, only PD7 and PD6 are available"]
    #[inline(always)]
    #[must_use]
    pub fn pd7(&mut self) -> PD7_W<7> {
        PD7_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PWR Port B pull-down control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pdcrb](index.html) module"]
pub struct PDCRB_SPEC;
impl crate::RegisterSpec for PDCRB_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pdcrb::R](R) reader structure"]
impl crate::Readable for PDCRB_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pdcrb::W](W) writer structure"]
impl crate::Writable for PDCRB_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PDCRB to value 0"]
impl crate::Resettable for PDCRB_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
