#[doc = "Register `EECR3` reader"]
pub struct R(crate::R<EECR3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EECR3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EECR3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EECR3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EECR3` writer"]
pub struct W(crate::W<EECR3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EECR3_SPEC>;
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
impl From<crate::W<EECR3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EECR3_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EE6F` reader - EE6F"]
pub type EE6F_R = crate::FieldReader<u8, EE6F_A>;
#[doc = "EE6F\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum EE6F_A {
    #[doc = "0: Filter disabled"]
    Disabled = 0,
    #[doc = "1: f_SAMPLING=f_HRTIM, N=2"]
    Div1N2 = 1,
    #[doc = "2: f_SAMPLING=f_HRTIM, N=4"]
    Div1N4 = 2,
    #[doc = "3: f_SAMPLING=f_HRTIM, N=8"]
    Div1N8 = 3,
    #[doc = "4: f_SAMPLING=f_HRTIM/2, N=6"]
    Div2N6 = 4,
    #[doc = "5: f_SAMPLING=f_HRTIM/2, N=8"]
    Div2N8 = 5,
    #[doc = "6: f_SAMPLING=f_HRTIM/4, N=6"]
    Div4N6 = 6,
    #[doc = "7: f_SAMPLING=f_HRTIM/4, N=8"]
    Div4N8 = 7,
    #[doc = "8: f_SAMPLING=f_HRTIM/8, N=6"]
    Div8N6 = 8,
    #[doc = "9: f_SAMPLING=f_HRTIM/8, N=8"]
    Div8N8 = 9,
    #[doc = "10: f_SAMPLING=f_HRTIM/16, N=5"]
    Div16N5 = 10,
    #[doc = "11: f_SAMPLING=f_HRTIM/16, N=6"]
    Div16N6 = 11,
    #[doc = "12: f_SAMPLING=f_HRTIM/16, N=8"]
    Div16N8 = 12,
    #[doc = "13: f_SAMPLING=f_HRTIM/32, N=5"]
    Div32N5 = 13,
    #[doc = "14: f_SAMPLING=f_HRTIM/32, N=6"]
    Div32N6 = 14,
    #[doc = "15: f_SAMPLING=f_HRTIM/32, N=8"]
    Div32N8 = 15,
}
impl From<EE6F_A> for u8 {
    #[inline(always)]
    fn from(variant: EE6F_A) -> Self {
        variant as _
    }
}
impl EE6F_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EE6F_A {
        match self.bits {
            0 => EE6F_A::Disabled,
            1 => EE6F_A::Div1N2,
            2 => EE6F_A::Div1N4,
            3 => EE6F_A::Div1N8,
            4 => EE6F_A::Div2N6,
            5 => EE6F_A::Div2N8,
            6 => EE6F_A::Div4N6,
            7 => EE6F_A::Div4N8,
            8 => EE6F_A::Div8N6,
            9 => EE6F_A::Div8N8,
            10 => EE6F_A::Div16N5,
            11 => EE6F_A::Div16N6,
            12 => EE6F_A::Div16N8,
            13 => EE6F_A::Div32N5,
            14 => EE6F_A::Div32N6,
            15 => EE6F_A::Div32N8,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == EE6F_A::Disabled
    }
    #[doc = "Checks if the value of the field is `Div1N2`"]
    #[inline(always)]
    pub fn is_div1_n2(&self) -> bool {
        *self == EE6F_A::Div1N2
    }
    #[doc = "Checks if the value of the field is `Div1N4`"]
    #[inline(always)]
    pub fn is_div1_n4(&self) -> bool {
        *self == EE6F_A::Div1N4
    }
    #[doc = "Checks if the value of the field is `Div1N8`"]
    #[inline(always)]
    pub fn is_div1_n8(&self) -> bool {
        *self == EE6F_A::Div1N8
    }
    #[doc = "Checks if the value of the field is `Div2N6`"]
    #[inline(always)]
    pub fn is_div2_n6(&self) -> bool {
        *self == EE6F_A::Div2N6
    }
    #[doc = "Checks if the value of the field is `Div2N8`"]
    #[inline(always)]
    pub fn is_div2_n8(&self) -> bool {
        *self == EE6F_A::Div2N8
    }
    #[doc = "Checks if the value of the field is `Div4N6`"]
    #[inline(always)]
    pub fn is_div4_n6(&self) -> bool {
        *self == EE6F_A::Div4N6
    }
    #[doc = "Checks if the value of the field is `Div4N8`"]
    #[inline(always)]
    pub fn is_div4_n8(&self) -> bool {
        *self == EE6F_A::Div4N8
    }
    #[doc = "Checks if the value of the field is `Div8N6`"]
    #[inline(always)]
    pub fn is_div8_n6(&self) -> bool {
        *self == EE6F_A::Div8N6
    }
    #[doc = "Checks if the value of the field is `Div8N8`"]
    #[inline(always)]
    pub fn is_div8_n8(&self) -> bool {
        *self == EE6F_A::Div8N8
    }
    #[doc = "Checks if the value of the field is `Div16N5`"]
    #[inline(always)]
    pub fn is_div16_n5(&self) -> bool {
        *self == EE6F_A::Div16N5
    }
    #[doc = "Checks if the value of the field is `Div16N6`"]
    #[inline(always)]
    pub fn is_div16_n6(&self) -> bool {
        *self == EE6F_A::Div16N6
    }
    #[doc = "Checks if the value of the field is `Div16N8`"]
    #[inline(always)]
    pub fn is_div16_n8(&self) -> bool {
        *self == EE6F_A::Div16N8
    }
    #[doc = "Checks if the value of the field is `Div32N5`"]
    #[inline(always)]
    pub fn is_div32_n5(&self) -> bool {
        *self == EE6F_A::Div32N5
    }
    #[doc = "Checks if the value of the field is `Div32N6`"]
    #[inline(always)]
    pub fn is_div32_n6(&self) -> bool {
        *self == EE6F_A::Div32N6
    }
    #[doc = "Checks if the value of the field is `Div32N8`"]
    #[inline(always)]
    pub fn is_div32_n8(&self) -> bool {
        *self == EE6F_A::Div32N8
    }
}
#[doc = "Field `EE6F` writer - EE6F"]
pub type EE6F_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, EECR3_SPEC, u8, EE6F_A, 4, O>;
impl<'a, const O: u8> EE6F_W<'a, O> {
    #[doc = "Filter disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(EE6F_A::Disabled)
    }
    #[doc = "f_SAMPLING=f_HRTIM, N=2"]
    #[inline(always)]
    pub fn div1_n2(self) -> &'a mut W {
        self.variant(EE6F_A::Div1N2)
    }
    #[doc = "f_SAMPLING=f_HRTIM, N=4"]
    #[inline(always)]
    pub fn div1_n4(self) -> &'a mut W {
        self.variant(EE6F_A::Div1N4)
    }
    #[doc = "f_SAMPLING=f_HRTIM, N=8"]
    #[inline(always)]
    pub fn div1_n8(self) -> &'a mut W {
        self.variant(EE6F_A::Div1N8)
    }
    #[doc = "f_SAMPLING=f_HRTIM/2, N=6"]
    #[inline(always)]
    pub fn div2_n6(self) -> &'a mut W {
        self.variant(EE6F_A::Div2N6)
    }
    #[doc = "f_SAMPLING=f_HRTIM/2, N=8"]
    #[inline(always)]
    pub fn div2_n8(self) -> &'a mut W {
        self.variant(EE6F_A::Div2N8)
    }
    #[doc = "f_SAMPLING=f_HRTIM/4, N=6"]
    #[inline(always)]
    pub fn div4_n6(self) -> &'a mut W {
        self.variant(EE6F_A::Div4N6)
    }
    #[doc = "f_SAMPLING=f_HRTIM/4, N=8"]
    #[inline(always)]
    pub fn div4_n8(self) -> &'a mut W {
        self.variant(EE6F_A::Div4N8)
    }
    #[doc = "f_SAMPLING=f_HRTIM/8, N=6"]
    #[inline(always)]
    pub fn div8_n6(self) -> &'a mut W {
        self.variant(EE6F_A::Div8N6)
    }
    #[doc = "f_SAMPLING=f_HRTIM/8, N=8"]
    #[inline(always)]
    pub fn div8_n8(self) -> &'a mut W {
        self.variant(EE6F_A::Div8N8)
    }
    #[doc = "f_SAMPLING=f_HRTIM/16, N=5"]
    #[inline(always)]
    pub fn div16_n5(self) -> &'a mut W {
        self.variant(EE6F_A::Div16N5)
    }
    #[doc = "f_SAMPLING=f_HRTIM/16, N=6"]
    #[inline(always)]
    pub fn div16_n6(self) -> &'a mut W {
        self.variant(EE6F_A::Div16N6)
    }
    #[doc = "f_SAMPLING=f_HRTIM/16, N=8"]
    #[inline(always)]
    pub fn div16_n8(self) -> &'a mut W {
        self.variant(EE6F_A::Div16N8)
    }
    #[doc = "f_SAMPLING=f_HRTIM/32, N=5"]
    #[inline(always)]
    pub fn div32_n5(self) -> &'a mut W {
        self.variant(EE6F_A::Div32N5)
    }
    #[doc = "f_SAMPLING=f_HRTIM/32, N=6"]
    #[inline(always)]
    pub fn div32_n6(self) -> &'a mut W {
        self.variant(EE6F_A::Div32N6)
    }
    #[doc = "f_SAMPLING=f_HRTIM/32, N=8"]
    #[inline(always)]
    pub fn div32_n8(self) -> &'a mut W {
        self.variant(EE6F_A::Div32N8)
    }
}
#[doc = "Field `EE7F` reader - EE7F"]
pub use EE6F_R as EE7F_R;
#[doc = "Field `EE8F` reader - EE8F"]
pub use EE6F_R as EE8F_R;
#[doc = "Field `EE9F` reader - EE9F"]
pub use EE6F_R as EE9F_R;
#[doc = "Field `EE10F` reader - EE10F"]
pub use EE6F_R as EE10F_R;
#[doc = "Field `EE7F` writer - EE7F"]
pub use EE6F_W as EE7F_W;
#[doc = "Field `EE8F` writer - EE8F"]
pub use EE6F_W as EE8F_W;
#[doc = "Field `EE9F` writer - EE9F"]
pub use EE6F_W as EE9F_W;
#[doc = "Field `EE10F` writer - EE10F"]
pub use EE6F_W as EE10F_W;
#[doc = "Field `EEVSD` reader - EEVSD"]
pub type EEVSD_R = crate::FieldReader<u8, EEVSD_A>;
#[doc = "EEVSD\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum EEVSD_A {
    #[doc = "0: f_EEVS=f_HRTIM"]
    Div1 = 0,
    #[doc = "1: f_EEVS=f_HRTIM/2"]
    Div2 = 1,
    #[doc = "2: f_EEVS=f_HRTIM/4"]
    Div4 = 2,
    #[doc = "3: f_EEVS=f_HRTIM/8"]
    Div8 = 3,
}
impl From<EEVSD_A> for u8 {
    #[inline(always)]
    fn from(variant: EEVSD_A) -> Self {
        variant as _
    }
}
impl EEVSD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EEVSD_A {
        match self.bits {
            0 => EEVSD_A::Div1,
            1 => EEVSD_A::Div2,
            2 => EEVSD_A::Div4,
            3 => EEVSD_A::Div8,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `Div1`"]
    #[inline(always)]
    pub fn is_div1(&self) -> bool {
        *self == EEVSD_A::Div1
    }
    #[doc = "Checks if the value of the field is `Div2`"]
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        *self == EEVSD_A::Div2
    }
    #[doc = "Checks if the value of the field is `Div4`"]
    #[inline(always)]
    pub fn is_div4(&self) -> bool {
        *self == EEVSD_A::Div4
    }
    #[doc = "Checks if the value of the field is `Div8`"]
    #[inline(always)]
    pub fn is_div8(&self) -> bool {
        *self == EEVSD_A::Div8
    }
}
#[doc = "Field `EEVSD` writer - EEVSD"]
pub type EEVSD_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, EECR3_SPEC, u8, EEVSD_A, 2, O>;
impl<'a, const O: u8> EEVSD_W<'a, O> {
    #[doc = "f_EEVS=f_HRTIM"]
    #[inline(always)]
    pub fn div1(self) -> &'a mut W {
        self.variant(EEVSD_A::Div1)
    }
    #[doc = "f_EEVS=f_HRTIM/2"]
    #[inline(always)]
    pub fn div2(self) -> &'a mut W {
        self.variant(EEVSD_A::Div2)
    }
    #[doc = "f_EEVS=f_HRTIM/4"]
    #[inline(always)]
    pub fn div4(self) -> &'a mut W {
        self.variant(EEVSD_A::Div4)
    }
    #[doc = "f_EEVS=f_HRTIM/8"]
    #[inline(always)]
    pub fn div8(self) -> &'a mut W {
        self.variant(EEVSD_A::Div8)
    }
}
impl R {
    #[doc = "Bits 0:3 - EE6F"]
    #[inline(always)]
    pub fn ee6f(&self) -> EE6F_R {
        EE6F_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 6:9 - EE7F"]
    #[inline(always)]
    pub fn ee7f(&self) -> EE7F_R {
        EE7F_R::new(((self.bits >> 6) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - EE8F"]
    #[inline(always)]
    pub fn ee8f(&self) -> EE8F_R {
        EE8F_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 18:21 - EE9F"]
    #[inline(always)]
    pub fn ee9f(&self) -> EE9F_R {
        EE9F_R::new(((self.bits >> 18) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - EE10F"]
    #[inline(always)]
    pub fn ee10f(&self) -> EE10F_R {
        EE10F_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 30:31 - EEVSD"]
    #[inline(always)]
    pub fn eevsd(&self) -> EEVSD_R {
        EEVSD_R::new(((self.bits >> 30) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - EE6F"]
    #[inline(always)]
    #[must_use]
    pub fn ee6f(&mut self) -> EE6F_W<0> {
        EE6F_W::new(self)
    }
    #[doc = "Bits 6:9 - EE7F"]
    #[inline(always)]
    #[must_use]
    pub fn ee7f(&mut self) -> EE7F_W<6> {
        EE7F_W::new(self)
    }
    #[doc = "Bits 12:15 - EE8F"]
    #[inline(always)]
    #[must_use]
    pub fn ee8f(&mut self) -> EE8F_W<12> {
        EE8F_W::new(self)
    }
    #[doc = "Bits 18:21 - EE9F"]
    #[inline(always)]
    #[must_use]
    pub fn ee9f(&mut self) -> EE9F_W<18> {
        EE9F_W::new(self)
    }
    #[doc = "Bits 24:27 - EE10F"]
    #[inline(always)]
    #[must_use]
    pub fn ee10f(&mut self) -> EE10F_W<24> {
        EE10F_W::new(self)
    }
    #[doc = "Bits 30:31 - EEVSD"]
    #[inline(always)]
    #[must_use]
    pub fn eevsd(&mut self) -> EEVSD_W<30> {
        EEVSD_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Timer External Event Control Register 3\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [eecr3](index.html) module"]
pub struct EECR3_SPEC;
impl crate::RegisterSpec for EECR3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [eecr3::R](R) reader structure"]
impl crate::Readable for EECR3_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [eecr3::W](W) writer structure"]
impl crate::Writable for EECR3_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets EECR3 to value 0"]
impl crate::Resettable for EECR3_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
