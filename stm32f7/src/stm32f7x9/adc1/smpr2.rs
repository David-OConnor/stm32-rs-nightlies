#[doc = "Register `SMPR2` reader"]
pub struct R(crate::R<SMPR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SMPR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SMPR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SMPR2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SMPR2` writer"]
pub struct W(crate::W<SMPR2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SMPR2_SPEC>;
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
impl From<crate::W<SMPR2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SMPR2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SMPx_x` reader - Sample time bits"]
pub type SMPX_X_R = crate::FieldReader<u32, SMPX_X_A>;
#[doc = "Sample time bits\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u32)]
pub enum SMPX_X_A {
    #[doc = "0: 3 cycles"]
    Cycles3 = 0,
    #[doc = "1: 15 cycles"]
    Cycles15 = 1,
    #[doc = "2: 28 cycles"]
    Cycles28 = 2,
    #[doc = "3: 56 cycles"]
    Cycles56 = 3,
    #[doc = "4: 84 cycles"]
    Cycles84 = 4,
    #[doc = "5: 112 cycles"]
    Cycles112 = 5,
    #[doc = "6: 144 cycles"]
    Cycles144 = 6,
    #[doc = "7: 480 cycles"]
    Cycles480 = 7,
}
impl From<SMPX_X_A> for u32 {
    #[inline(always)]
    fn from(variant: SMPX_X_A) -> Self {
        variant as _
    }
}
impl SMPX_X_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<SMPX_X_A> {
        match self.bits {
            0 => Some(SMPX_X_A::Cycles3),
            1 => Some(SMPX_X_A::Cycles15),
            2 => Some(SMPX_X_A::Cycles28),
            3 => Some(SMPX_X_A::Cycles56),
            4 => Some(SMPX_X_A::Cycles84),
            5 => Some(SMPX_X_A::Cycles112),
            6 => Some(SMPX_X_A::Cycles144),
            7 => Some(SMPX_X_A::Cycles480),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `Cycles3`"]
    #[inline(always)]
    pub fn is_cycles3(&self) -> bool {
        *self == SMPX_X_A::Cycles3
    }
    #[doc = "Checks if the value of the field is `Cycles15`"]
    #[inline(always)]
    pub fn is_cycles15(&self) -> bool {
        *self == SMPX_X_A::Cycles15
    }
    #[doc = "Checks if the value of the field is `Cycles28`"]
    #[inline(always)]
    pub fn is_cycles28(&self) -> bool {
        *self == SMPX_X_A::Cycles28
    }
    #[doc = "Checks if the value of the field is `Cycles56`"]
    #[inline(always)]
    pub fn is_cycles56(&self) -> bool {
        *self == SMPX_X_A::Cycles56
    }
    #[doc = "Checks if the value of the field is `Cycles84`"]
    #[inline(always)]
    pub fn is_cycles84(&self) -> bool {
        *self == SMPX_X_A::Cycles84
    }
    #[doc = "Checks if the value of the field is `Cycles112`"]
    #[inline(always)]
    pub fn is_cycles112(&self) -> bool {
        *self == SMPX_X_A::Cycles112
    }
    #[doc = "Checks if the value of the field is `Cycles144`"]
    #[inline(always)]
    pub fn is_cycles144(&self) -> bool {
        *self == SMPX_X_A::Cycles144
    }
    #[doc = "Checks if the value of the field is `Cycles480`"]
    #[inline(always)]
    pub fn is_cycles480(&self) -> bool {
        *self == SMPX_X_A::Cycles480
    }
}
#[doc = "Field `SMPx_x` writer - Sample time bits"]
pub type SMPX_X_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SMPR2_SPEC, u32, SMPX_X_A, 32, O>;
impl<'a, const O: u8> SMPX_X_W<'a, O> {
    #[doc = "3 cycles"]
    #[inline(always)]
    pub fn cycles3(self) -> &'a mut W {
        self.variant(SMPX_X_A::Cycles3)
    }
    #[doc = "15 cycles"]
    #[inline(always)]
    pub fn cycles15(self) -> &'a mut W {
        self.variant(SMPX_X_A::Cycles15)
    }
    #[doc = "28 cycles"]
    #[inline(always)]
    pub fn cycles28(self) -> &'a mut W {
        self.variant(SMPX_X_A::Cycles28)
    }
    #[doc = "56 cycles"]
    #[inline(always)]
    pub fn cycles56(self) -> &'a mut W {
        self.variant(SMPX_X_A::Cycles56)
    }
    #[doc = "84 cycles"]
    #[inline(always)]
    pub fn cycles84(self) -> &'a mut W {
        self.variant(SMPX_X_A::Cycles84)
    }
    #[doc = "112 cycles"]
    #[inline(always)]
    pub fn cycles112(self) -> &'a mut W {
        self.variant(SMPX_X_A::Cycles112)
    }
    #[doc = "144 cycles"]
    #[inline(always)]
    pub fn cycles144(self) -> &'a mut W {
        self.variant(SMPX_X_A::Cycles144)
    }
    #[doc = "480 cycles"]
    #[inline(always)]
    pub fn cycles480(self) -> &'a mut W {
        self.variant(SMPX_X_A::Cycles480)
    }
}
impl R {
    #[doc = "Bits 0:31 - Sample time bits"]
    #[inline(always)]
    pub fn smpx_x(&self) -> SMPX_X_R {
        SMPX_X_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Sample time bits"]
    #[inline(always)]
    #[must_use]
    pub fn smpx_x(&mut self) -> SMPX_X_W<0> {
        SMPX_X_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "sample time register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [smpr2](index.html) module"]
pub struct SMPR2_SPEC;
impl crate::RegisterSpec for SMPR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [smpr2::R](R) reader structure"]
impl crate::Readable for SMPR2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [smpr2::W](W) writer structure"]
impl crate::Writable for SMPR2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SMPR2 to value 0"]
impl crate::Resettable for SMPR2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
