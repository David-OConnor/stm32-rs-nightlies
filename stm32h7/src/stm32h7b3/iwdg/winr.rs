#[doc = "Register `WINR` reader"]
pub struct R(crate::R<WINR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<WINR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<WINR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<WINR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `WINR` writer"]
pub struct W(crate::W<WINR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<WINR_SPEC>;
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
impl From<crate::W<WINR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<WINR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `WIN` reader - Watchdog counter window value These bits are write access protected see Section23.3.6. These bits contain the high limit of the window value to be compared to the downcounter. To prevent a reset, the downcounter must be reloaded when its value is lower than the window register value and greater than 0x0 The WVU bit in the IWDG_SR register must be reset in order to be able to change the reload value. Note: Reading this register returns the reload value from the VDD voltage domain. This value may not be valid if a write operation to this register is ongoing. For this reason the value read from this register is valid only when the WVU bit in the IWDG_SR register is reset."]
pub type WIN_R = crate::FieldReader<u16, u16>;
#[doc = "Field `WIN` writer - Watchdog counter window value These bits are write access protected see Section23.3.6. These bits contain the high limit of the window value to be compared to the downcounter. To prevent a reset, the downcounter must be reloaded when its value is lower than the window register value and greater than 0x0 The WVU bit in the IWDG_SR register must be reset in order to be able to change the reload value. Note: Reading this register returns the reload value from the VDD voltage domain. This value may not be valid if a write operation to this register is ongoing. For this reason the value read from this register is valid only when the WVU bit in the IWDG_SR register is reset."]
pub type WIN_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, WINR_SPEC, u16, u16, 12, O>;
impl R {
    #[doc = "Bits 0:11 - Watchdog counter window value These bits are write access protected see Section23.3.6. These bits contain the high limit of the window value to be compared to the downcounter. To prevent a reset, the downcounter must be reloaded when its value is lower than the window register value and greater than 0x0 The WVU bit in the IWDG_SR register must be reset in order to be able to change the reload value. Note: Reading this register returns the reload value from the VDD voltage domain. This value may not be valid if a write operation to this register is ongoing. For this reason the value read from this register is valid only when the WVU bit in the IWDG_SR register is reset."]
    #[inline(always)]
    pub fn win(&self) -> WIN_R {
        WIN_R::new((self.bits & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - Watchdog counter window value These bits are write access protected see Section23.3.6. These bits contain the high limit of the window value to be compared to the downcounter. To prevent a reset, the downcounter must be reloaded when its value is lower than the window register value and greater than 0x0 The WVU bit in the IWDG_SR register must be reset in order to be able to change the reload value. Note: Reading this register returns the reload value from the VDD voltage domain. This value may not be valid if a write operation to this register is ongoing. For this reason the value read from this register is valid only when the WVU bit in the IWDG_SR register is reset."]
    #[inline(always)]
    #[must_use]
    pub fn win(&mut self) -> WIN_W<0> {
        WIN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Window register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [winr](index.html) module"]
pub struct WINR_SPEC;
impl crate::RegisterSpec for WINR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [winr::R](R) reader structure"]
impl crate::Readable for WINR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [winr::W](W) writer structure"]
impl crate::Writable for WINR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets WINR to value 0x0fff"]
impl crate::Resettable for WINR_SPEC {
    const RESET_VALUE: Self::Ux = 0x0fff;
}
