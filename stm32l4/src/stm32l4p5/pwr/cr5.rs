#[doc = "Register `CR5` reader"]
pub struct R(crate::R<CR5_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CR5_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CR5_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CR5_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CR5` writer"]
pub struct W(crate::W<CR5_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CR5_SPEC>;
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
impl From<crate::W<CR5_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CR5_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `R1MODE` reader - Main regulator Range 1 mode"]
pub type R1MODE_R = crate::BitReader<R1MODE_A>;
#[doc = "Main regulator Range 1 mode\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum R1MODE_A {
    #[doc = "0: Main regulator in Range 1 boost mode"]
    BoostMode = 0,
    #[doc = "1: Main regulator in Range 1 normal mode"]
    NormalMode = 1,
}
impl From<R1MODE_A> for bool {
    #[inline(always)]
    fn from(variant: R1MODE_A) -> Self {
        variant as u8 != 0
    }
}
impl R1MODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> R1MODE_A {
        match self.bits {
            false => R1MODE_A::BoostMode,
            true => R1MODE_A::NormalMode,
        }
    }
    #[doc = "Checks if the value of the field is `BoostMode`"]
    #[inline(always)]
    pub fn is_boost_mode(&self) -> bool {
        *self == R1MODE_A::BoostMode
    }
    #[doc = "Checks if the value of the field is `NormalMode`"]
    #[inline(always)]
    pub fn is_normal_mode(&self) -> bool {
        *self == R1MODE_A::NormalMode
    }
}
#[doc = "Field `R1MODE` writer - Main regulator Range 1 mode"]
pub type R1MODE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR5_SPEC, R1MODE_A, O>;
impl<'a, const O: u8> R1MODE_W<'a, O> {
    #[doc = "Main regulator in Range 1 boost mode"]
    #[inline(always)]
    pub fn boost_mode(self) -> &'a mut W {
        self.variant(R1MODE_A::BoostMode)
    }
    #[doc = "Main regulator in Range 1 normal mode"]
    #[inline(always)]
    pub fn normal_mode(self) -> &'a mut W {
        self.variant(R1MODE_A::NormalMode)
    }
}
impl R {
    #[doc = "Bit 8 - Main regulator Range 1 mode"]
    #[inline(always)]
    pub fn r1mode(&self) -> R1MODE_R {
        R1MODE_R::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 8 - Main regulator Range 1 mode"]
    #[inline(always)]
    #[must_use]
    pub fn r1mode(&mut self) -> R1MODE_W<8> {
        R1MODE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "control register 5\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cr5](index.html) module"]
pub struct CR5_SPEC;
impl crate::RegisterSpec for CR5_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cr5::R](R) reader structure"]
impl crate::Readable for CR5_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cr5::W](W) writer structure"]
impl crate::Writable for CR5_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CR5 to value 0x0100"]
impl crate::Resettable for CR5_SPEC {
    const RESET_VALUE: Self::Ux = 0x0100;
}
