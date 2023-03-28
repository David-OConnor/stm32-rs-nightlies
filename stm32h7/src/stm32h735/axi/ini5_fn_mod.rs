#[doc = "Register `INI5_FN_MOD` reader"]
pub struct R(crate::R<INI5_FN_MOD_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INI5_FN_MOD_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INI5_FN_MOD_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INI5_FN_MOD_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `INI5_FN_MOD` writer"]
pub struct W(crate::W<INI5_FN_MOD_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INI5_FN_MOD_SPEC>;
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
impl From<crate::W<INI5_FN_MOD_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INI5_FN_MOD_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `READ_ISS_OVERRIDE` reader - Override ASIB read issuing capability"]
pub type READ_ISS_OVERRIDE_R = crate::BitReader<READ_ISS_OVERRIDE_A>;
#[doc = "Override ASIB read issuing capability\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum READ_ISS_OVERRIDE_A {
    #[doc = "0: Normal ASIB read issuing capability"]
    Normal = 0,
    #[doc = "1: Force ASIB read issuing capability to 1"]
    Force1 = 1,
}
impl From<READ_ISS_OVERRIDE_A> for bool {
    #[inline(always)]
    fn from(variant: READ_ISS_OVERRIDE_A) -> Self {
        variant as u8 != 0
    }
}
impl READ_ISS_OVERRIDE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> READ_ISS_OVERRIDE_A {
        match self.bits {
            false => READ_ISS_OVERRIDE_A::Normal,
            true => READ_ISS_OVERRIDE_A::Force1,
        }
    }
    #[doc = "Checks if the value of the field is `Normal`"]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == READ_ISS_OVERRIDE_A::Normal
    }
    #[doc = "Checks if the value of the field is `Force1`"]
    #[inline(always)]
    pub fn is_force1(&self) -> bool {
        *self == READ_ISS_OVERRIDE_A::Force1
    }
}
#[doc = "Field `READ_ISS_OVERRIDE` writer - Override ASIB read issuing capability"]
pub type READ_ISS_OVERRIDE_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, INI5_FN_MOD_SPEC, READ_ISS_OVERRIDE_A, O>;
impl<'a, const O: u8> READ_ISS_OVERRIDE_W<'a, O> {
    #[doc = "Normal ASIB read issuing capability"]
    #[inline(always)]
    pub fn normal(self) -> &'a mut W {
        self.variant(READ_ISS_OVERRIDE_A::Normal)
    }
    #[doc = "Force ASIB read issuing capability to 1"]
    #[inline(always)]
    pub fn force1(self) -> &'a mut W {
        self.variant(READ_ISS_OVERRIDE_A::Force1)
    }
}
#[doc = "Field `WRITE_ISS_OVERRIDE` reader - Override ASIB write issuing capability"]
pub type WRITE_ISS_OVERRIDE_R = crate::BitReader<WRITE_ISS_OVERRIDE_A>;
#[doc = "Override ASIB write issuing capability\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WRITE_ISS_OVERRIDE_A {
    #[doc = "0: Normal ASIB write issuing capability"]
    Normal = 0,
    #[doc = "1: Force ASIB write issuing capability to 1"]
    Force1 = 1,
}
impl From<WRITE_ISS_OVERRIDE_A> for bool {
    #[inline(always)]
    fn from(variant: WRITE_ISS_OVERRIDE_A) -> Self {
        variant as u8 != 0
    }
}
impl WRITE_ISS_OVERRIDE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WRITE_ISS_OVERRIDE_A {
        match self.bits {
            false => WRITE_ISS_OVERRIDE_A::Normal,
            true => WRITE_ISS_OVERRIDE_A::Force1,
        }
    }
    #[doc = "Checks if the value of the field is `Normal`"]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == WRITE_ISS_OVERRIDE_A::Normal
    }
    #[doc = "Checks if the value of the field is `Force1`"]
    #[inline(always)]
    pub fn is_force1(&self) -> bool {
        *self == WRITE_ISS_OVERRIDE_A::Force1
    }
}
#[doc = "Field `WRITE_ISS_OVERRIDE` writer - Override ASIB write issuing capability"]
pub type WRITE_ISS_OVERRIDE_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, INI5_FN_MOD_SPEC, WRITE_ISS_OVERRIDE_A, O>;
impl<'a, const O: u8> WRITE_ISS_OVERRIDE_W<'a, O> {
    #[doc = "Normal ASIB write issuing capability"]
    #[inline(always)]
    pub fn normal(self) -> &'a mut W {
        self.variant(WRITE_ISS_OVERRIDE_A::Normal)
    }
    #[doc = "Force ASIB write issuing capability to 1"]
    #[inline(always)]
    pub fn force1(self) -> &'a mut W {
        self.variant(WRITE_ISS_OVERRIDE_A::Force1)
    }
}
impl R {
    #[doc = "Bit 0 - Override ASIB read issuing capability"]
    #[inline(always)]
    pub fn read_iss_override(&self) -> READ_ISS_OVERRIDE_R {
        READ_ISS_OVERRIDE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Override ASIB write issuing capability"]
    #[inline(always)]
    pub fn write_iss_override(&self) -> WRITE_ISS_OVERRIDE_R {
        WRITE_ISS_OVERRIDE_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Override ASIB read issuing capability"]
    #[inline(always)]
    #[must_use]
    pub fn read_iss_override(&mut self) -> READ_ISS_OVERRIDE_W<0> {
        READ_ISS_OVERRIDE_W::new(self)
    }
    #[doc = "Bit 1 - Override ASIB write issuing capability"]
    #[inline(always)]
    #[must_use]
    pub fn write_iss_override(&mut self) -> WRITE_ISS_OVERRIDE_W<1> {
        WRITE_ISS_OVERRIDE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "AXI interconnect - INI x issuing functionality modification register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ini5_fn_mod](index.html) module"]
pub struct INI5_FN_MOD_SPEC;
impl crate::RegisterSpec for INI5_FN_MOD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ini5_fn_mod::R](R) reader structure"]
impl crate::Readable for INI5_FN_MOD_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ini5_fn_mod::W](W) writer structure"]
impl crate::Writable for INI5_FN_MOD_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets INI5_FN_MOD to value 0x04"]
impl crate::Resettable for INI5_FN_MOD_SPEC {
    const RESET_VALUE: Self::Ux = 0x04;
}
