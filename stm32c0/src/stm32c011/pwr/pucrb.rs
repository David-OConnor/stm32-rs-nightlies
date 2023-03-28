#[doc = "Register `PUCRB` reader"]
pub struct R(crate::R<PUCRB_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PUCRB_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PUCRB_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PUCRB_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PUCRB` writer"]
pub struct W(crate::W<PUCRB_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PUCRB_SPEC>;
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
impl From<crate::W<PUCRB_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PUCRB_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PU6` reader - Port B pull-up bit i (i = 15 to 0) Setting PUi bit while the corresponding PDi bit is zero and the APC bit of the PWR_CR3 register is set activates a pull-up device on the PB\\[i\\]
I/O. On STM32C011xx, only PU7 and PU6 are available"]
pub type PU6_R = crate::BitReader<bool>;
#[doc = "Field `PU6` writer - Port B pull-up bit i (i = 15 to 0) Setting PUi bit while the corresponding PDi bit is zero and the APC bit of the PWR_CR3 register is set activates a pull-up device on the PB\\[i\\]
I/O. On STM32C011xx, only PU7 and PU6 are available"]
pub type PU6_W<'a, const O: u8> = crate::BitWriter<'a, u32, PUCRB_SPEC, bool, O>;
#[doc = "Field `PU7` reader - Port B pull-up bit i (i = 15 to 0) Setting PUi bit while the corresponding PDi bit is zero and the APC bit of the PWR_CR3 register is set activates a pull-up device on the PB\\[i\\]
I/O. On STM32C011xx, only PU7 and PU6 are available"]
pub type PU7_R = crate::BitReader<bool>;
#[doc = "Field `PU7` writer - Port B pull-up bit i (i = 15 to 0) Setting PUi bit while the corresponding PDi bit is zero and the APC bit of the PWR_CR3 register is set activates a pull-up device on the PB\\[i\\]
I/O. On STM32C011xx, only PU7 and PU6 are available"]
pub type PU7_W<'a, const O: u8> = crate::BitWriter<'a, u32, PUCRB_SPEC, bool, O>;
impl R {
    #[doc = "Bit 6 - Port B pull-up bit i (i = 15 to 0) Setting PUi bit while the corresponding PDi bit is zero and the APC bit of the PWR_CR3 register is set activates a pull-up device on the PB\\[i\\]
I/O. On STM32C011xx, only PU7 and PU6 are available"]
    #[inline(always)]
    pub fn pu6(&self) -> PU6_R {
        PU6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Port B pull-up bit i (i = 15 to 0) Setting PUi bit while the corresponding PDi bit is zero and the APC bit of the PWR_CR3 register is set activates a pull-up device on the PB\\[i\\]
I/O. On STM32C011xx, only PU7 and PU6 are available"]
    #[inline(always)]
    pub fn pu7(&self) -> PU7_R {
        PU7_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 6 - Port B pull-up bit i (i = 15 to 0) Setting PUi bit while the corresponding PDi bit is zero and the APC bit of the PWR_CR3 register is set activates a pull-up device on the PB\\[i\\]
I/O. On STM32C011xx, only PU7 and PU6 are available"]
    #[inline(always)]
    #[must_use]
    pub fn pu6(&mut self) -> PU6_W<6> {
        PU6_W::new(self)
    }
    #[doc = "Bit 7 - Port B pull-up bit i (i = 15 to 0) Setting PUi bit while the corresponding PDi bit is zero and the APC bit of the PWR_CR3 register is set activates a pull-up device on the PB\\[i\\]
I/O. On STM32C011xx, only PU7 and PU6 are available"]
    #[inline(always)]
    #[must_use]
    pub fn pu7(&mut self) -> PU7_W<7> {
        PU7_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PWR Port B pull-up control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pucrb](index.html) module"]
pub struct PUCRB_SPEC;
impl crate::RegisterSpec for PUCRB_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pucrb::R](R) reader structure"]
impl crate::Readable for PUCRB_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pucrb::W](W) writer structure"]
impl crate::Writable for PUCRB_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PUCRB to value 0"]
impl crate::Resettable for PUCRB_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
