#[doc = "Register `CFGR2` reader"]
pub struct R(crate::R<CFGR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CFGR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CFGR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CFGR2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CFGR2` writer"]
pub struct W(crate::W<CFGR2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CFGR2_SPEC>;
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
impl From<crate::W<CFGR2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CFGR2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CKMODE` reader - ADC clock mode"]
pub type CKMODE_R = crate::FieldReader<u8, CKMODE_A>;
#[doc = "ADC clock mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CKMODE_A {
    #[doc = "0: Asynchronous clock mode"]
    Adcclk = 0,
    #[doc = "1: Synchronous clock mode (PCLK/2)"]
    PclkDiv2 = 1,
    #[doc = "2: Sychronous clock mode (PCLK/4)"]
    PclkDiv4 = 2,
}
impl From<CKMODE_A> for u8 {
    #[inline(always)]
    fn from(variant: CKMODE_A) -> Self {
        variant as _
    }
}
impl CKMODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<CKMODE_A> {
        match self.bits {
            0 => Some(CKMODE_A::Adcclk),
            1 => Some(CKMODE_A::PclkDiv2),
            2 => Some(CKMODE_A::PclkDiv4),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `Adcclk`"]
    #[inline(always)]
    pub fn is_adcclk(&self) -> bool {
        *self == CKMODE_A::Adcclk
    }
    #[doc = "Checks if the value of the field is `PclkDiv2`"]
    #[inline(always)]
    pub fn is_pclk_div2(&self) -> bool {
        *self == CKMODE_A::PclkDiv2
    }
    #[doc = "Checks if the value of the field is `PclkDiv4`"]
    #[inline(always)]
    pub fn is_pclk_div4(&self) -> bool {
        *self == CKMODE_A::PclkDiv4
    }
}
#[doc = "Field `CKMODE` writer - ADC clock mode"]
pub type CKMODE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CFGR2_SPEC, u8, CKMODE_A, 2, O>;
impl<'a, const O: u8> CKMODE_W<'a, O> {
    #[doc = "Asynchronous clock mode"]
    #[inline(always)]
    pub fn adcclk(self) -> &'a mut W {
        self.variant(CKMODE_A::Adcclk)
    }
    #[doc = "Synchronous clock mode (PCLK/2)"]
    #[inline(always)]
    pub fn pclk_div2(self) -> &'a mut W {
        self.variant(CKMODE_A::PclkDiv2)
    }
    #[doc = "Sychronous clock mode (PCLK/4)"]
    #[inline(always)]
    pub fn pclk_div4(self) -> &'a mut W {
        self.variant(CKMODE_A::PclkDiv4)
    }
}
impl R {
    #[doc = "Bits 30:31 - ADC clock mode"]
    #[inline(always)]
    pub fn ckmode(&self) -> CKMODE_R {
        CKMODE_R::new(((self.bits >> 30) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 30:31 - ADC clock mode"]
    #[inline(always)]
    #[must_use]
    pub fn ckmode(&mut self) -> CKMODE_W<30> {
        CKMODE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "configuration register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cfgr2](index.html) module"]
pub struct CFGR2_SPEC;
impl crate::RegisterSpec for CFGR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cfgr2::R](R) reader structure"]
impl crate::Readable for CFGR2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cfgr2::W](W) writer structure"]
impl crate::Writable for CFGR2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CFGR2 to value 0x8000"]
impl crate::Resettable for CFGR2_SPEC {
    const RESET_VALUE: Self::Ux = 0x8000;
}
