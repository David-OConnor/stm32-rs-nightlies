#[doc = "Register `I3C_IBIDR` reader"]
pub struct R(crate::R<I3C_IBIDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<I3C_IBIDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<I3C_IBIDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<I3C_IBIDR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `I3C_IBIDR` writer"]
pub struct W(crate::W<I3C_IBIDR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<I3C_IBIDR_SPEC>;
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
impl From<crate::W<I3C_IBIDR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<I3C_IBIDR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `IBIDB0` reader - 8-bit IBI payload data (earliest byte on I3C bus, i.e. MDB\\[7:0\\]
mandatory data byte)."]
pub type IBIDB0_R = crate::FieldReader<u8, u8>;
#[doc = "Field `IBIDB0` writer - 8-bit IBI payload data (earliest byte on I3C bus, i.e. MDB\\[7:0\\]
mandatory data byte)."]
pub type IBIDB0_W<'a, const O: u8> = crate::FieldWriter<'a, u32, I3C_IBIDR_SPEC, u8, u8, 8, O>;
#[doc = "Field `IBIDB1` reader - 8-bit IBI payload data (next byte on I3C bus after IBIDB0\\[7:0\\])."]
pub type IBIDB1_R = crate::FieldReader<u8, u8>;
#[doc = "Field `IBIDB1` writer - 8-bit IBI payload data (next byte on I3C bus after IBIDB0\\[7:0\\])."]
pub type IBIDB1_W<'a, const O: u8> = crate::FieldWriter<'a, u32, I3C_IBIDR_SPEC, u8, u8, 8, O>;
#[doc = "Field `IBIDB2` reader - 8-bit IBI payload data (next byte on I3C bus after IBIDB1\\[7:0\\])."]
pub type IBIDB2_R = crate::FieldReader<u8, u8>;
#[doc = "Field `IBIDB2` writer - 8-bit IBI payload data (next byte on I3C bus after IBIDB1\\[7:0\\])."]
pub type IBIDB2_W<'a, const O: u8> = crate::FieldWriter<'a, u32, I3C_IBIDR_SPEC, u8, u8, 8, O>;
#[doc = "Field `IBIDB3` reader - 8-bit IBI payload data (latest byte on I3C bus)."]
pub type IBIDB3_R = crate::FieldReader<u8, u8>;
#[doc = "Field `IBIDB3` writer - 8-bit IBI payload data (latest byte on I3C bus)."]
pub type IBIDB3_W<'a, const O: u8> = crate::FieldWriter<'a, u32, I3C_IBIDR_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - 8-bit IBI payload data (earliest byte on I3C bus, i.e. MDB\\[7:0\\]
mandatory data byte)."]
    #[inline(always)]
    pub fn ibidb0(&self) -> IBIDB0_R {
        IBIDB0_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - 8-bit IBI payload data (next byte on I3C bus after IBIDB0\\[7:0\\])."]
    #[inline(always)]
    pub fn ibidb1(&self) -> IBIDB1_R {
        IBIDB1_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - 8-bit IBI payload data (next byte on I3C bus after IBIDB1\\[7:0\\])."]
    #[inline(always)]
    pub fn ibidb2(&self) -> IBIDB2_R {
        IBIDB2_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - 8-bit IBI payload data (latest byte on I3C bus)."]
    #[inline(always)]
    pub fn ibidb3(&self) -> IBIDB3_R {
        IBIDB3_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - 8-bit IBI payload data (earliest byte on I3C bus, i.e. MDB\\[7:0\\]
mandatory data byte)."]
    #[inline(always)]
    #[must_use]
    pub fn ibidb0(&mut self) -> IBIDB0_W<0> {
        IBIDB0_W::new(self)
    }
    #[doc = "Bits 8:15 - 8-bit IBI payload data (next byte on I3C bus after IBIDB0\\[7:0\\])."]
    #[inline(always)]
    #[must_use]
    pub fn ibidb1(&mut self) -> IBIDB1_W<8> {
        IBIDB1_W::new(self)
    }
    #[doc = "Bits 16:23 - 8-bit IBI payload data (next byte on I3C bus after IBIDB1\\[7:0\\])."]
    #[inline(always)]
    #[must_use]
    pub fn ibidb2(&mut self) -> IBIDB2_W<16> {
        IBIDB2_W::new(self)
    }
    #[doc = "Bits 24:31 - 8-bit IBI payload data (latest byte on I3C bus)."]
    #[inline(always)]
    #[must_use]
    pub fn ibidb3(&mut self) -> IBIDB3_W<24> {
        IBIDB3_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "I3C IBI payload data register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [i3c_ibidr](index.html) module"]
pub struct I3C_IBIDR_SPEC;
impl crate::RegisterSpec for I3C_IBIDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [i3c_ibidr::R](R) reader structure"]
impl crate::Readable for I3C_IBIDR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [i3c_ibidr::W](W) writer structure"]
impl crate::Writable for I3C_IBIDR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets I3C_IBIDR to value 0"]
impl crate::Resettable for I3C_IBIDR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
