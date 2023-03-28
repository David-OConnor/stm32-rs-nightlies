#[doc = "Register `PUCRF` reader"]
pub struct R(crate::R<PUCRF_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PUCRF_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PUCRF_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PUCRF_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PUCRF` writer"]
pub struct W(crate::W<PUCRF_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PUCRF_SPEC>;
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
impl From<crate::W<PUCRF_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PUCRF_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PU2` reader - Port F pull-up bit i (i = 2 to 0) Setting PUi bit while the corresponding PDi bit is zero and the APC bit of the PWR_CR3 register is set activates a pull-up device on the PF\\[i\\]
I/O. On STM32C011xx, only PU2 is available."]
pub type PU2_R = crate::BitReader<bool>;
#[doc = "Field `PU2` writer - Port F pull-up bit i (i = 2 to 0) Setting PUi bit while the corresponding PDi bit is zero and the APC bit of the PWR_CR3 register is set activates a pull-up device on the PF\\[i\\]
I/O. On STM32C011xx, only PU2 is available."]
pub type PU2_W<'a, const O: u8> = crate::BitWriter<'a, u32, PUCRF_SPEC, bool, O>;
impl R {
    #[doc = "Bit 2 - Port F pull-up bit i (i = 2 to 0) Setting PUi bit while the corresponding PDi bit is zero and the APC bit of the PWR_CR3 register is set activates a pull-up device on the PF\\[i\\]
I/O. On STM32C011xx, only PU2 is available."]
    #[inline(always)]
    pub fn pu2(&self) -> PU2_R {
        PU2_R::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 2 - Port F pull-up bit i (i = 2 to 0) Setting PUi bit while the corresponding PDi bit is zero and the APC bit of the PWR_CR3 register is set activates a pull-up device on the PF\\[i\\]
I/O. On STM32C011xx, only PU2 is available."]
    #[inline(always)]
    #[must_use]
    pub fn pu2(&mut self) -> PU2_W<2> {
        PU2_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PWR Port F pull-up control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pucrf](index.html) module"]
pub struct PUCRF_SPEC;
impl crate::RegisterSpec for PUCRF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pucrf::R](R) reader structure"]
impl crate::Readable for PUCRF_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pucrf::W](W) writer structure"]
impl crate::Writable for PUCRF_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PUCRF to value 0"]
impl crate::Resettable for PUCRF_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
