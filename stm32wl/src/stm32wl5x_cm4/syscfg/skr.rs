#[doc = "Register `SKR` writer"]
pub struct W(crate::W<SKR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SKR_SPEC>;
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
impl From<crate::W<SKR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SKR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "SRAM2 write protection key for software erase\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum KEY_AW {
    #[doc = "17: Activate SRAM2ER bits write protection"]
    WriteProtect = 17,
    #[doc = "83: Step 2 to remove SRAM2ER bits write protection"]
    Step2 = 83,
    #[doc = "202: Step 1 to remove SRAM2ER bits write protection"]
    Step1 = 202,
}
impl From<KEY_AW> for u8 {
    #[inline(always)]
    fn from(variant: KEY_AW) -> Self {
        variant as _
    }
}
#[doc = "Field `KEY` writer - SRAM2 write protection key for software erase"]
pub type KEY_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SKR_SPEC, u8, KEY_AW, 8, O>;
impl<'a, const O: u8> KEY_W<'a, O> {
    #[doc = "Activate SRAM2ER bits write protection"]
    #[inline(always)]
    pub fn write_protect(self) -> &'a mut W {
        self.variant(KEY_AW::WriteProtect)
    }
    #[doc = "Step 2 to remove SRAM2ER bits write protection"]
    #[inline(always)]
    pub fn step2(self) -> &'a mut W {
        self.variant(KEY_AW::Step2)
    }
    #[doc = "Step 1 to remove SRAM2ER bits write protection"]
    #[inline(always)]
    pub fn step1(self) -> &'a mut W {
        self.variant(KEY_AW::Step1)
    }
}
impl W {
    #[doc = "Bits 0:7 - SRAM2 write protection key for software erase"]
    #[inline(always)]
    #[must_use]
    pub fn key(&mut self) -> KEY_W<0> {
        KEY_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SKR\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [skr](index.html) module"]
pub struct SKR_SPEC;
impl crate::RegisterSpec for SKR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [skr::W](W) writer structure"]
impl crate::Writable for SKR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SKR to value 0"]
impl crate::Resettable for SKR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
