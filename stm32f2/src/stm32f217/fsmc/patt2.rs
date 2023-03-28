#[doc = "Register `PATT2` reader"]
pub struct R(crate::R<PATT2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PATT2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PATT2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PATT2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PATT2` writer"]
pub struct W(crate::W<PATT2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PATT2_SPEC>;
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
impl From<crate::W<PATT2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PATT2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ATTSET` reader - Attribute memory x setup time"]
pub type ATTSET_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ATTSET` writer - Attribute memory x setup time"]
pub type ATTSET_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PATT2_SPEC, u8, u8, 8, O>;
#[doc = "Field `ATTWAIT` reader - Attribute memory x wait time"]
pub type ATTWAIT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ATTWAIT` writer - Attribute memory x wait time"]
pub type ATTWAIT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PATT2_SPEC, u8, u8, 8, O>;
#[doc = "Field `ATTHOLD` reader - Attribute memory x hold time"]
pub type ATTHOLD_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ATTHOLD` writer - Attribute memory x hold time"]
pub type ATTHOLD_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PATT2_SPEC, u8, u8, 8, O>;
#[doc = "Field `ATTHIZ` reader - Attribute memory x databus HiZ time"]
pub type ATTHIZ_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ATTHIZ` writer - Attribute memory x databus HiZ time"]
pub type ATTHIZ_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PATT2_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - Attribute memory x setup time"]
    #[inline(always)]
    pub fn attset(&self) -> ATTSET_R {
        ATTSET_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Attribute memory x wait time"]
    #[inline(always)]
    pub fn attwait(&self) -> ATTWAIT_R {
        ATTWAIT_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Attribute memory x hold time"]
    #[inline(always)]
    pub fn atthold(&self) -> ATTHOLD_R {
        ATTHOLD_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Attribute memory x databus HiZ time"]
    #[inline(always)]
    pub fn atthiz(&self) -> ATTHIZ_R {
        ATTHIZ_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Attribute memory x setup time"]
    #[inline(always)]
    #[must_use]
    pub fn attset(&mut self) -> ATTSET_W<0> {
        ATTSET_W::new(self)
    }
    #[doc = "Bits 8:15 - Attribute memory x wait time"]
    #[inline(always)]
    #[must_use]
    pub fn attwait(&mut self) -> ATTWAIT_W<8> {
        ATTWAIT_W::new(self)
    }
    #[doc = "Bits 16:23 - Attribute memory x hold time"]
    #[inline(always)]
    #[must_use]
    pub fn atthold(&mut self) -> ATTHOLD_W<16> {
        ATTHOLD_W::new(self)
    }
    #[doc = "Bits 24:31 - Attribute memory x databus HiZ time"]
    #[inline(always)]
    #[must_use]
    pub fn atthiz(&mut self) -> ATTHIZ_W<24> {
        ATTHIZ_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Attribute memory space timing register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [patt2](index.html) module"]
pub struct PATT2_SPEC;
impl crate::RegisterSpec for PATT2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [patt2::R](R) reader structure"]
impl crate::Readable for PATT2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [patt2::W](W) writer structure"]
impl crate::Writable for PATT2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PATT2 to value 0xfcfc_fcfc"]
impl crate::Resettable for PATT2_SPEC {
    const RESET_VALUE: Self::Ux = 0xfcfc_fcfc;
}
