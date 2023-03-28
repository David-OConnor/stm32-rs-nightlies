#[doc = "Register `DCR2` reader"]
pub struct R(crate::R<DCR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DCR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DCR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DCR2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DCR2` writer"]
pub struct W(crate::W<DCR2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DCR2_SPEC>;
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
impl From<crate::W<DCR2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DCR2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PRESCALER` reader - Clock prescaler"]
pub type PRESCALER_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PRESCALER` writer - Clock prescaler"]
pub type PRESCALER_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, DCR2_SPEC, u8, u8, 8, O>;
#[doc = "Field `WRAPSIZE` reader - Wrap size"]
pub type WRAPSIZE_R = crate::FieldReader<u8, WRAPSIZE_A>;
#[doc = "Wrap size\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum WRAPSIZE_A {
    #[doc = "0: Wrapped reads are not supported by the memory"]
    NoWrappingSupport = 0,
    #[doc = "2: External memory supports wrap size of 16 bytes"]
    WrappingSize16 = 2,
    #[doc = "3: External memory supports wrap size of 16 bytes"]
    WrappingSize32 = 3,
    #[doc = "4: External memory supports wrap size of 16 bytes"]
    WrappingSize64 = 4,
    #[doc = "5: External memory supports wrap size of 16 bytes"]
    WrappingSize128 = 5,
}
impl From<WRAPSIZE_A> for u8 {
    #[inline(always)]
    fn from(variant: WRAPSIZE_A) -> Self {
        variant as _
    }
}
impl WRAPSIZE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<WRAPSIZE_A> {
        match self.bits {
            0 => Some(WRAPSIZE_A::NoWrappingSupport),
            2 => Some(WRAPSIZE_A::WrappingSize16),
            3 => Some(WRAPSIZE_A::WrappingSize32),
            4 => Some(WRAPSIZE_A::WrappingSize64),
            5 => Some(WRAPSIZE_A::WrappingSize128),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `NoWrappingSupport`"]
    #[inline(always)]
    pub fn is_no_wrapping_support(&self) -> bool {
        *self == WRAPSIZE_A::NoWrappingSupport
    }
    #[doc = "Checks if the value of the field is `WrappingSize16`"]
    #[inline(always)]
    pub fn is_wrapping_size16(&self) -> bool {
        *self == WRAPSIZE_A::WrappingSize16
    }
    #[doc = "Checks if the value of the field is `WrappingSize32`"]
    #[inline(always)]
    pub fn is_wrapping_size32(&self) -> bool {
        *self == WRAPSIZE_A::WrappingSize32
    }
    #[doc = "Checks if the value of the field is `WrappingSize64`"]
    #[inline(always)]
    pub fn is_wrapping_size64(&self) -> bool {
        *self == WRAPSIZE_A::WrappingSize64
    }
    #[doc = "Checks if the value of the field is `WrappingSize128`"]
    #[inline(always)]
    pub fn is_wrapping_size128(&self) -> bool {
        *self == WRAPSIZE_A::WrappingSize128
    }
}
#[doc = "Field `WRAPSIZE` writer - Wrap size"]
pub type WRAPSIZE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DCR2_SPEC, u8, WRAPSIZE_A, 3, O>;
impl<'a, const O: u8> WRAPSIZE_W<'a, O> {
    #[doc = "Wrapped reads are not supported by the memory"]
    #[inline(always)]
    pub fn no_wrapping_support(self) -> &'a mut W {
        self.variant(WRAPSIZE_A::NoWrappingSupport)
    }
    #[doc = "External memory supports wrap size of 16 bytes"]
    #[inline(always)]
    pub fn wrapping_size16(self) -> &'a mut W {
        self.variant(WRAPSIZE_A::WrappingSize16)
    }
    #[doc = "External memory supports wrap size of 16 bytes"]
    #[inline(always)]
    pub fn wrapping_size32(self) -> &'a mut W {
        self.variant(WRAPSIZE_A::WrappingSize32)
    }
    #[doc = "External memory supports wrap size of 16 bytes"]
    #[inline(always)]
    pub fn wrapping_size64(self) -> &'a mut W {
        self.variant(WRAPSIZE_A::WrappingSize64)
    }
    #[doc = "External memory supports wrap size of 16 bytes"]
    #[inline(always)]
    pub fn wrapping_size128(self) -> &'a mut W {
        self.variant(WRAPSIZE_A::WrappingSize128)
    }
}
impl R {
    #[doc = "Bits 0:7 - Clock prescaler"]
    #[inline(always)]
    pub fn prescaler(&self) -> PRESCALER_R {
        PRESCALER_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 16:18 - Wrap size"]
    #[inline(always)]
    pub fn wrapsize(&self) -> WRAPSIZE_R {
        WRAPSIZE_R::new(((self.bits >> 16) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Clock prescaler"]
    #[inline(always)]
    #[must_use]
    pub fn prescaler(&mut self) -> PRESCALER_W<0> {
        PRESCALER_W::new(self)
    }
    #[doc = "Bits 16:18 - Wrap size"]
    #[inline(always)]
    #[must_use]
    pub fn wrapsize(&mut self) -> WRAPSIZE_W<16> {
        WRAPSIZE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "device configuration register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dcr2](index.html) module"]
pub struct DCR2_SPEC;
impl crate::RegisterSpec for DCR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dcr2::R](R) reader structure"]
impl crate::Readable for DCR2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dcr2::W](W) writer structure"]
impl crate::Writable for DCR2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DCR2 to value 0"]
impl crate::Resettable for DCR2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
