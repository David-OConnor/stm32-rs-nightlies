#[doc = "Register `EECR2` reader"]
pub struct R(crate::R<EECR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EECR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EECR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EECR2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EECR2` writer"]
pub struct W(crate::W<EECR2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EECR2_SPEC>;
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
impl From<crate::W<EECR2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EECR2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EE6SRC` reader - External Event 6 Source"]
pub type EE6SRC_R = crate::FieldReader<u8, EE6SRC_A>;
#[doc = "External Event 6 Source\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum EE6SRC_A {
    #[doc = "0: Source 1"]
    Src1 = 0,
    #[doc = "1: Source 2"]
    Src2 = 1,
    #[doc = "2: Source 3"]
    Src3 = 2,
    #[doc = "3: Source 4"]
    Src4 = 3,
}
impl From<EE6SRC_A> for u8 {
    #[inline(always)]
    fn from(variant: EE6SRC_A) -> Self {
        variant as _
    }
}
impl EE6SRC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EE6SRC_A {
        match self.bits {
            0 => EE6SRC_A::Src1,
            1 => EE6SRC_A::Src2,
            2 => EE6SRC_A::Src3,
            3 => EE6SRC_A::Src4,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `Src1`"]
    #[inline(always)]
    pub fn is_src1(&self) -> bool {
        *self == EE6SRC_A::Src1
    }
    #[doc = "Checks if the value of the field is `Src2`"]
    #[inline(always)]
    pub fn is_src2(&self) -> bool {
        *self == EE6SRC_A::Src2
    }
    #[doc = "Checks if the value of the field is `Src3`"]
    #[inline(always)]
    pub fn is_src3(&self) -> bool {
        *self == EE6SRC_A::Src3
    }
    #[doc = "Checks if the value of the field is `Src4`"]
    #[inline(always)]
    pub fn is_src4(&self) -> bool {
        *self == EE6SRC_A::Src4
    }
}
#[doc = "Field `EE6SRC` writer - External Event 6 Source"]
pub type EE6SRC_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, EECR2_SPEC, u8, EE6SRC_A, 2, O>;
impl<'a, const O: u8> EE6SRC_W<'a, O> {
    #[doc = "Source 1"]
    #[inline(always)]
    pub fn src1(self) -> &'a mut W {
        self.variant(EE6SRC_A::Src1)
    }
    #[doc = "Source 2"]
    #[inline(always)]
    pub fn src2(self) -> &'a mut W {
        self.variant(EE6SRC_A::Src2)
    }
    #[doc = "Source 3"]
    #[inline(always)]
    pub fn src3(self) -> &'a mut W {
        self.variant(EE6SRC_A::Src3)
    }
    #[doc = "Source 4"]
    #[inline(always)]
    pub fn src4(self) -> &'a mut W {
        self.variant(EE6SRC_A::Src4)
    }
}
#[doc = "Field `EE6POL` reader - External Event 6 Polarity"]
pub type EE6POL_R = crate::BitReader<EE6POL_A>;
#[doc = "External Event 6 Polarity\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EE6POL_A {
    #[doc = "0: External event is active high"]
    ActiveHigh = 0,
    #[doc = "1: External event is active low"]
    ActiveLow = 1,
}
impl From<EE6POL_A> for bool {
    #[inline(always)]
    fn from(variant: EE6POL_A) -> Self {
        variant as u8 != 0
    }
}
impl EE6POL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EE6POL_A {
        match self.bits {
            false => EE6POL_A::ActiveHigh,
            true => EE6POL_A::ActiveLow,
        }
    }
    #[doc = "Checks if the value of the field is `ActiveHigh`"]
    #[inline(always)]
    pub fn is_active_high(&self) -> bool {
        *self == EE6POL_A::ActiveHigh
    }
    #[doc = "Checks if the value of the field is `ActiveLow`"]
    #[inline(always)]
    pub fn is_active_low(&self) -> bool {
        *self == EE6POL_A::ActiveLow
    }
}
#[doc = "Field `EE6POL` writer - External Event 6 Polarity"]
pub type EE6POL_W<'a, const O: u8> = crate::BitWriter<'a, u32, EECR2_SPEC, EE6POL_A, O>;
impl<'a, const O: u8> EE6POL_W<'a, O> {
    #[doc = "External event is active high"]
    #[inline(always)]
    pub fn active_high(self) -> &'a mut W {
        self.variant(EE6POL_A::ActiveHigh)
    }
    #[doc = "External event is active low"]
    #[inline(always)]
    pub fn active_low(self) -> &'a mut W {
        self.variant(EE6POL_A::ActiveLow)
    }
}
#[doc = "Field `EE6SNS` reader - External Event 6 Sensitivity"]
pub type EE6SNS_R = crate::FieldReader<u8, EE6SNS_A>;
#[doc = "External Event 6 Sensitivity\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum EE6SNS_A {
    #[doc = "0: On active level defined by EExPOL bit"]
    Active = 0,
    #[doc = "1: Rising edge"]
    Rising = 1,
    #[doc = "2: Falling edge"]
    Falling = 2,
    #[doc = "3: Both edges"]
    Both = 3,
}
impl From<EE6SNS_A> for u8 {
    #[inline(always)]
    fn from(variant: EE6SNS_A) -> Self {
        variant as _
    }
}
impl EE6SNS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EE6SNS_A {
        match self.bits {
            0 => EE6SNS_A::Active,
            1 => EE6SNS_A::Rising,
            2 => EE6SNS_A::Falling,
            3 => EE6SNS_A::Both,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `Active`"]
    #[inline(always)]
    pub fn is_active(&self) -> bool {
        *self == EE6SNS_A::Active
    }
    #[doc = "Checks if the value of the field is `Rising`"]
    #[inline(always)]
    pub fn is_rising(&self) -> bool {
        *self == EE6SNS_A::Rising
    }
    #[doc = "Checks if the value of the field is `Falling`"]
    #[inline(always)]
    pub fn is_falling(&self) -> bool {
        *self == EE6SNS_A::Falling
    }
    #[doc = "Checks if the value of the field is `Both`"]
    #[inline(always)]
    pub fn is_both(&self) -> bool {
        *self == EE6SNS_A::Both
    }
}
#[doc = "Field `EE6SNS` writer - External Event 6 Sensitivity"]
pub type EE6SNS_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, EECR2_SPEC, u8, EE6SNS_A, 2, O>;
impl<'a, const O: u8> EE6SNS_W<'a, O> {
    #[doc = "On active level defined by EExPOL bit"]
    #[inline(always)]
    pub fn active(self) -> &'a mut W {
        self.variant(EE6SNS_A::Active)
    }
    #[doc = "Rising edge"]
    #[inline(always)]
    pub fn rising(self) -> &'a mut W {
        self.variant(EE6SNS_A::Rising)
    }
    #[doc = "Falling edge"]
    #[inline(always)]
    pub fn falling(self) -> &'a mut W {
        self.variant(EE6SNS_A::Falling)
    }
    #[doc = "Both edges"]
    #[inline(always)]
    pub fn both(self) -> &'a mut W {
        self.variant(EE6SNS_A::Both)
    }
}
#[doc = "Field `EE7POL` reader - External Event 7 Polarity"]
pub use EE6POL_R as EE7POL_R;
#[doc = "Field `EE8POL` reader - External Event 8 Polarity"]
pub use EE6POL_R as EE8POL_R;
#[doc = "Field `EE9POL` reader - External Event 9 Polarity"]
pub use EE6POL_R as EE9POL_R;
#[doc = "Field `EE10POL` reader - External Event 10 Polarity"]
pub use EE6POL_R as EE10POL_R;
#[doc = "Field `EE7POL` writer - External Event 7 Polarity"]
pub use EE6POL_W as EE7POL_W;
#[doc = "Field `EE8POL` writer - External Event 8 Polarity"]
pub use EE6POL_W as EE8POL_W;
#[doc = "Field `EE9POL` writer - External Event 9 Polarity"]
pub use EE6POL_W as EE9POL_W;
#[doc = "Field `EE10POL` writer - External Event 10 Polarity"]
pub use EE6POL_W as EE10POL_W;
#[doc = "Field `EE7SNS` reader - External Event 7 Sensitivity"]
pub use EE6SNS_R as EE7SNS_R;
#[doc = "Field `EE8SNS` reader - External Event 8 Sensitivity"]
pub use EE6SNS_R as EE8SNS_R;
#[doc = "Field `EE9SNS` reader - External Event 9 Sensitivity"]
pub use EE6SNS_R as EE9SNS_R;
#[doc = "Field `EE10SNS` reader - External Event 10 Sensitivity"]
pub use EE6SNS_R as EE10SNS_R;
#[doc = "Field `EE7SNS` writer - External Event 7 Sensitivity"]
pub use EE6SNS_W as EE7SNS_W;
#[doc = "Field `EE8SNS` writer - External Event 8 Sensitivity"]
pub use EE6SNS_W as EE8SNS_W;
#[doc = "Field `EE9SNS` writer - External Event 9 Sensitivity"]
pub use EE6SNS_W as EE9SNS_W;
#[doc = "Field `EE10SNS` writer - External Event 10 Sensitivity"]
pub use EE6SNS_W as EE10SNS_W;
#[doc = "Field `EE7SRC` reader - External Event 7 Source"]
pub use EE6SRC_R as EE7SRC_R;
#[doc = "Field `EE8SRC` reader - External Event 8 Source"]
pub use EE6SRC_R as EE8SRC_R;
#[doc = "Field `EE9SRC` reader - External Event 9 Source"]
pub use EE6SRC_R as EE9SRC_R;
#[doc = "Field `EE10SRC` reader - External Event 10 Source"]
pub use EE6SRC_R as EE10SRC_R;
#[doc = "Field `EE7SRC` writer - External Event 7 Source"]
pub use EE6SRC_W as EE7SRC_W;
#[doc = "Field `EE8SRC` writer - External Event 8 Source"]
pub use EE6SRC_W as EE8SRC_W;
#[doc = "Field `EE9SRC` writer - External Event 9 Source"]
pub use EE6SRC_W as EE9SRC_W;
#[doc = "Field `EE10SRC` writer - External Event 10 Source"]
pub use EE6SRC_W as EE10SRC_W;
impl R {
    #[doc = "Bits 0:1 - External Event 6 Source"]
    #[inline(always)]
    pub fn ee6src(&self) -> EE6SRC_R {
        EE6SRC_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2 - External Event 6 Polarity"]
    #[inline(always)]
    pub fn ee6pol(&self) -> EE6POL_R {
        EE6POL_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:4 - External Event 6 Sensitivity"]
    #[inline(always)]
    pub fn ee6sns(&self) -> EE6SNS_R {
        EE6SNS_R::new(((self.bits >> 3) & 3) as u8)
    }
    #[doc = "Bits 6:7 - External Event 7 Source"]
    #[inline(always)]
    pub fn ee7src(&self) -> EE7SRC_R {
        EE7SRC_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bit 8 - External Event 7 Polarity"]
    #[inline(always)]
    pub fn ee7pol(&self) -> EE7POL_R {
        EE7POL_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 9:10 - External Event 7 Sensitivity"]
    #[inline(always)]
    pub fn ee7sns(&self) -> EE7SNS_R {
        EE7SNS_R::new(((self.bits >> 9) & 3) as u8)
    }
    #[doc = "Bits 12:13 - External Event 8 Source"]
    #[inline(always)]
    pub fn ee8src(&self) -> EE8SRC_R {
        EE8SRC_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bit 14 - External Event 8 Polarity"]
    #[inline(always)]
    pub fn ee8pol(&self) -> EE8POL_R {
        EE8POL_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bits 15:16 - External Event 8 Sensitivity"]
    #[inline(always)]
    pub fn ee8sns(&self) -> EE8SNS_R {
        EE8SNS_R::new(((self.bits >> 15) & 3) as u8)
    }
    #[doc = "Bits 18:19 - External Event 9 Source"]
    #[inline(always)]
    pub fn ee9src(&self) -> EE9SRC_R {
        EE9SRC_R::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bit 20 - External Event 9 Polarity"]
    #[inline(always)]
    pub fn ee9pol(&self) -> EE9POL_R {
        EE9POL_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bits 21:22 - External Event 9 Sensitivity"]
    #[inline(always)]
    pub fn ee9sns(&self) -> EE9SNS_R {
        EE9SNS_R::new(((self.bits >> 21) & 3) as u8)
    }
    #[doc = "Bits 24:25 - External Event 10 Source"]
    #[inline(always)]
    pub fn ee10src(&self) -> EE10SRC_R {
        EE10SRC_R::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bit 26 - External Event 10 Polarity"]
    #[inline(always)]
    pub fn ee10pol(&self) -> EE10POL_R {
        EE10POL_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bits 27:28 - External Event 10 Sensitivity"]
    #[inline(always)]
    pub fn ee10sns(&self) -> EE10SNS_R {
        EE10SNS_R::new(((self.bits >> 27) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - External Event 6 Source"]
    #[inline(always)]
    #[must_use]
    pub fn ee6src(&mut self) -> EE6SRC_W<0> {
        EE6SRC_W::new(self)
    }
    #[doc = "Bit 2 - External Event 6 Polarity"]
    #[inline(always)]
    #[must_use]
    pub fn ee6pol(&mut self) -> EE6POL_W<2> {
        EE6POL_W::new(self)
    }
    #[doc = "Bits 3:4 - External Event 6 Sensitivity"]
    #[inline(always)]
    #[must_use]
    pub fn ee6sns(&mut self) -> EE6SNS_W<3> {
        EE6SNS_W::new(self)
    }
    #[doc = "Bits 6:7 - External Event 7 Source"]
    #[inline(always)]
    #[must_use]
    pub fn ee7src(&mut self) -> EE7SRC_W<6> {
        EE7SRC_W::new(self)
    }
    #[doc = "Bit 8 - External Event 7 Polarity"]
    #[inline(always)]
    #[must_use]
    pub fn ee7pol(&mut self) -> EE7POL_W<8> {
        EE7POL_W::new(self)
    }
    #[doc = "Bits 9:10 - External Event 7 Sensitivity"]
    #[inline(always)]
    #[must_use]
    pub fn ee7sns(&mut self) -> EE7SNS_W<9> {
        EE7SNS_W::new(self)
    }
    #[doc = "Bits 12:13 - External Event 8 Source"]
    #[inline(always)]
    #[must_use]
    pub fn ee8src(&mut self) -> EE8SRC_W<12> {
        EE8SRC_W::new(self)
    }
    #[doc = "Bit 14 - External Event 8 Polarity"]
    #[inline(always)]
    #[must_use]
    pub fn ee8pol(&mut self) -> EE8POL_W<14> {
        EE8POL_W::new(self)
    }
    #[doc = "Bits 15:16 - External Event 8 Sensitivity"]
    #[inline(always)]
    #[must_use]
    pub fn ee8sns(&mut self) -> EE8SNS_W<15> {
        EE8SNS_W::new(self)
    }
    #[doc = "Bits 18:19 - External Event 9 Source"]
    #[inline(always)]
    #[must_use]
    pub fn ee9src(&mut self) -> EE9SRC_W<18> {
        EE9SRC_W::new(self)
    }
    #[doc = "Bit 20 - External Event 9 Polarity"]
    #[inline(always)]
    #[must_use]
    pub fn ee9pol(&mut self) -> EE9POL_W<20> {
        EE9POL_W::new(self)
    }
    #[doc = "Bits 21:22 - External Event 9 Sensitivity"]
    #[inline(always)]
    #[must_use]
    pub fn ee9sns(&mut self) -> EE9SNS_W<21> {
        EE9SNS_W::new(self)
    }
    #[doc = "Bits 24:25 - External Event 10 Source"]
    #[inline(always)]
    #[must_use]
    pub fn ee10src(&mut self) -> EE10SRC_W<24> {
        EE10SRC_W::new(self)
    }
    #[doc = "Bit 26 - External Event 10 Polarity"]
    #[inline(always)]
    #[must_use]
    pub fn ee10pol(&mut self) -> EE10POL_W<26> {
        EE10POL_W::new(self)
    }
    #[doc = "Bits 27:28 - External Event 10 Sensitivity"]
    #[inline(always)]
    #[must_use]
    pub fn ee10sns(&mut self) -> EE10SNS_W<27> {
        EE10SNS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Timer External Event Control Register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [eecr2](index.html) module"]
pub struct EECR2_SPEC;
impl crate::RegisterSpec for EECR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [eecr2::R](R) reader structure"]
impl crate::Readable for EECR2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [eecr2::W](W) writer structure"]
impl crate::Writable for EECR2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets EECR2 to value 0"]
impl crate::Resettable for EECR2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
