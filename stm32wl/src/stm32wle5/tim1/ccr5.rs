#[doc = "Register `CCR5` reader"]
pub struct R(crate::R<CCR5_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CCR5_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CCR5_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CCR5_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CCR5` writer"]
pub struct W(crate::W<CCR5_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CCR5_SPEC>;
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
impl From<crate::W<CCR5_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CCR5_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CCR5` reader - Capture/Compare 5 value"]
pub type CCR5_R = crate::FieldReader<u16, u16>;
#[doc = "Field `CCR5` writer - Capture/Compare 5 value"]
pub type CCR5_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, CCR5_SPEC, u16, u16, 16, O>;
#[doc = "Field `GC5C1` reader - Group Channel 5 and Channel 1"]
pub type GC5C1_R = crate::BitReader<GC5C1_A>;
#[doc = "Group Channel 5 and Channel 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GC5C1_A {
    #[doc = "0: No effect of OC5REF on OC1REFC"]
    Disabled = 0,
    #[doc = "1: OC1REFC is the logical AND of OC1REFC and OC5REF"]
    Enabled = 1,
}
impl From<GC5C1_A> for bool {
    #[inline(always)]
    fn from(variant: GC5C1_A) -> Self {
        variant as u8 != 0
    }
}
impl GC5C1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GC5C1_A {
        match self.bits {
            false => GC5C1_A::Disabled,
            true => GC5C1_A::Enabled,
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == GC5C1_A::Disabled
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == GC5C1_A::Enabled
    }
}
#[doc = "Field `GC5C1` writer - Group Channel 5 and Channel 1"]
pub type GC5C1_W<'a, const O: u8> = crate::BitWriter<'a, u32, CCR5_SPEC, GC5C1_A, O>;
impl<'a, const O: u8> GC5C1_W<'a, O> {
    #[doc = "No effect of OC5REF on OC1REFC"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(GC5C1_A::Disabled)
    }
    #[doc = "OC1REFC is the logical AND of OC1REFC and OC5REF"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(GC5C1_A::Enabled)
    }
}
#[doc = "Field `GC5C2` reader - Group Channel 5 and Channel 2"]
pub type GC5C2_R = crate::BitReader<GC5C2_A>;
#[doc = "Group Channel 5 and Channel 2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GC5C2_A {
    #[doc = "0: No effect of OC5REF on OC2REFC"]
    Disabled = 0,
    #[doc = "1: OC2REFC is the logical AND of OC2REFC and OC5REF"]
    Enabled = 1,
}
impl From<GC5C2_A> for bool {
    #[inline(always)]
    fn from(variant: GC5C2_A) -> Self {
        variant as u8 != 0
    }
}
impl GC5C2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GC5C2_A {
        match self.bits {
            false => GC5C2_A::Disabled,
            true => GC5C2_A::Enabled,
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == GC5C2_A::Disabled
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == GC5C2_A::Enabled
    }
}
#[doc = "Field `GC5C2` writer - Group Channel 5 and Channel 2"]
pub type GC5C2_W<'a, const O: u8> = crate::BitWriter<'a, u32, CCR5_SPEC, GC5C2_A, O>;
impl<'a, const O: u8> GC5C2_W<'a, O> {
    #[doc = "No effect of OC5REF on OC2REFC"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(GC5C2_A::Disabled)
    }
    #[doc = "OC2REFC is the logical AND of OC2REFC and OC5REF"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(GC5C2_A::Enabled)
    }
}
#[doc = "Field `GC5C3` reader - Group Channel 5 and Channel 3"]
pub type GC5C3_R = crate::BitReader<GC5C3_A>;
#[doc = "Group Channel 5 and Channel 3\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GC5C3_A {
    #[doc = "0: No effect of OC5REF on OC3REFC"]
    Disabled = 0,
    #[doc = "1: OC3REFC is the logical AND of OC3REFC and OC5REF"]
    Enabled = 1,
}
impl From<GC5C3_A> for bool {
    #[inline(always)]
    fn from(variant: GC5C3_A) -> Self {
        variant as u8 != 0
    }
}
impl GC5C3_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GC5C3_A {
        match self.bits {
            false => GC5C3_A::Disabled,
            true => GC5C3_A::Enabled,
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == GC5C3_A::Disabled
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == GC5C3_A::Enabled
    }
}
#[doc = "Field `GC5C3` writer - Group Channel 5 and Channel 3"]
pub type GC5C3_W<'a, const O: u8> = crate::BitWriter<'a, u32, CCR5_SPEC, GC5C3_A, O>;
impl<'a, const O: u8> GC5C3_W<'a, O> {
    #[doc = "No effect of OC5REF on OC3REFC"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(GC5C3_A::Disabled)
    }
    #[doc = "OC3REFC is the logical AND of OC3REFC and OC5REF"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(GC5C3_A::Enabled)
    }
}
impl R {
    #[doc = "Bits 0:15 - Capture/Compare 5 value"]
    #[inline(always)]
    pub fn ccr5(&self) -> CCR5_R {
        CCR5_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bit 29 - Group Channel 5 and Channel 1"]
    #[inline(always)]
    pub fn gc5c1(&self) -> GC5C1_R {
        GC5C1_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Group Channel 5 and Channel 2"]
    #[inline(always)]
    pub fn gc5c2(&self) -> GC5C2_R {
        GC5C2_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Group Channel 5 and Channel 3"]
    #[inline(always)]
    pub fn gc5c3(&self) -> GC5C3_R {
        GC5C3_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:15 - Capture/Compare 5 value"]
    #[inline(always)]
    #[must_use]
    pub fn ccr5(&mut self) -> CCR5_W<0> {
        CCR5_W::new(self)
    }
    #[doc = "Bit 29 - Group Channel 5 and Channel 1"]
    #[inline(always)]
    #[must_use]
    pub fn gc5c1(&mut self) -> GC5C1_W<29> {
        GC5C1_W::new(self)
    }
    #[doc = "Bit 30 - Group Channel 5 and Channel 2"]
    #[inline(always)]
    #[must_use]
    pub fn gc5c2(&mut self) -> GC5C2_W<30> {
        GC5C2_W::new(self)
    }
    #[doc = "Bit 31 - Group Channel 5 and Channel 3"]
    #[inline(always)]
    #[must_use]
    pub fn gc5c3(&mut self) -> GC5C3_W<31> {
        GC5C3_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "capture/compare register 5\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ccr5](index.html) module"]
pub struct CCR5_SPEC;
impl crate::RegisterSpec for CCR5_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ccr5::R](R) reader structure"]
impl crate::Readable for CCR5_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ccr5::W](W) writer structure"]
impl crate::Writable for CCR5_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CCR5 to value 0"]
impl crate::Resettable for CCR5_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
