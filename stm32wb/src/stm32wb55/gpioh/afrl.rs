#[doc = "Register `AFRL` reader"]
pub struct R(crate::R<AFRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<AFRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<AFRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<AFRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `AFRL` writer"]
pub struct W(crate::W<AFRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<AFRL_SPEC>;
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
impl From<crate::W<AFRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<AFRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `AFSEL0` reader - Alternate function selection for port x bit y (y = 0..7)"]
pub type AFSEL0_R = crate::FieldReader<u8, u8>;
#[doc = "Field `AFSEL0` writer - Alternate function selection for port x bit y (y = 0..7)"]
pub type AFSEL0_W<'a, const O: u8> = crate::FieldWriter<'a, u32, AFRL_SPEC, u8, u8, 4, O>;
#[doc = "Field `AFSEL1` reader - Alternate function selection for port x bit y (y = 0..7)"]
pub type AFSEL1_R = crate::FieldReader<u8, u8>;
#[doc = "Field `AFSEL1` writer - Alternate function selection for port x bit y (y = 0..7)"]
pub type AFSEL1_W<'a, const O: u8> = crate::FieldWriter<'a, u32, AFRL_SPEC, u8, u8, 4, O>;
#[doc = "Field `AFSEL3` reader - Alternate function selection for port x bit y (y = 0..7)"]
pub type AFSEL3_R = crate::FieldReader<u8, u8>;
#[doc = "Field `AFSEL3` writer - Alternate function selection for port x bit y (y = 0..7)"]
pub type AFSEL3_W<'a, const O: u8> = crate::FieldWriter<'a, u32, AFRL_SPEC, u8, u8, 4, O>;
impl R {
    #[doc = "Bits 0:3 - Alternate function selection for port x bit y (y = 0..7)"]
    #[inline(always)]
    pub fn afsel0(&self) -> AFSEL0_R {
        AFSEL0_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Alternate function selection for port x bit y (y = 0..7)"]
    #[inline(always)]
    pub fn afsel1(&self) -> AFSEL1_R {
        AFSEL1_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - Alternate function selection for port x bit y (y = 0..7)"]
    #[inline(always)]
    pub fn afsel3(&self) -> AFSEL3_R {
        AFSEL3_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Alternate function selection for port x bit y (y = 0..7)"]
    #[inline(always)]
    #[must_use]
    pub fn afsel0(&mut self) -> AFSEL0_W<0> {
        AFSEL0_W::new(self)
    }
    #[doc = "Bits 4:7 - Alternate function selection for port x bit y (y = 0..7)"]
    #[inline(always)]
    #[must_use]
    pub fn afsel1(&mut self) -> AFSEL1_W<4> {
        AFSEL1_W::new(self)
    }
    #[doc = "Bits 12:15 - Alternate function selection for port x bit y (y = 0..7)"]
    #[inline(always)]
    #[must_use]
    pub fn afsel3(&mut self) -> AFSEL3_W<12> {
        AFSEL3_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "GPIO alternate function low register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [afrl](index.html) module"]
pub struct AFRL_SPEC;
impl crate::RegisterSpec for AFRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [afrl::R](R) reader structure"]
impl crate::Readable for AFRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [afrl::W](W) writer structure"]
impl crate::Writable for AFRL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets AFRL to value 0"]
impl crate::Resettable for AFRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
