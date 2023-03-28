#[doc = "Register `OAR1` reader"]
pub struct R(crate::R<OAR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OAR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OAR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OAR1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `OAR1` writer"]
pub struct W(crate::W<OAR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OAR1_SPEC>;
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
impl From<crate::W<OAR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<OAR1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `OA1` reader - Interface own slave address 7-bit addressing mode: OA1\\[7:1\\]
contains the 7-bit own slave address. The bits OA1\\[9\\], OA1\\[8\\]
and OA1\\[0\\]
are don't care. 10-bit addressing mode: OA1\\[9:0\\]
contains the 10-bit own slave address. Note: These bits can be written only when OA1EN=0."]
pub type OA1_R = crate::FieldReader<u16, u16>;
#[doc = "Field `OA1` writer - Interface own slave address 7-bit addressing mode: OA1\\[7:1\\]
contains the 7-bit own slave address. The bits OA1\\[9\\], OA1\\[8\\]
and OA1\\[0\\]
are don't care. 10-bit addressing mode: OA1\\[9:0\\]
contains the 10-bit own slave address. Note: These bits can be written only when OA1EN=0."]
pub type OA1_W<'a, const O: u8> = crate::FieldWriter<'a, u32, OAR1_SPEC, u16, u16, 10, O>;
#[doc = "Field `OA1MODE` reader - Own Address 1 10-bit mode Note: This bit can be written only when OA1EN=0."]
pub type OA1MODE_R = crate::BitReader<bool>;
#[doc = "Field `OA1MODE` writer - Own Address 1 10-bit mode Note: This bit can be written only when OA1EN=0."]
pub type OA1MODE_W<'a, const O: u8> = crate::BitWriter<'a, u32, OAR1_SPEC, bool, O>;
#[doc = "Field `OA1EN` reader - Own Address 1 enable"]
pub type OA1EN_R = crate::BitReader<bool>;
#[doc = "Field `OA1EN` writer - Own Address 1 enable"]
pub type OA1EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, OAR1_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:9 - Interface own slave address 7-bit addressing mode: OA1\\[7:1\\]
contains the 7-bit own slave address. The bits OA1\\[9\\], OA1\\[8\\]
and OA1\\[0\\]
are don't care. 10-bit addressing mode: OA1\\[9:0\\]
contains the 10-bit own slave address. Note: These bits can be written only when OA1EN=0."]
    #[inline(always)]
    pub fn oa1(&self) -> OA1_R {
        OA1_R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bit 10 - Own Address 1 10-bit mode Note: This bit can be written only when OA1EN=0."]
    #[inline(always)]
    pub fn oa1mode(&self) -> OA1MODE_R {
        OA1MODE_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 15 - Own Address 1 enable"]
    #[inline(always)]
    pub fn oa1en(&self) -> OA1EN_R {
        OA1EN_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:9 - Interface own slave address 7-bit addressing mode: OA1\\[7:1\\]
contains the 7-bit own slave address. The bits OA1\\[9\\], OA1\\[8\\]
and OA1\\[0\\]
are don't care. 10-bit addressing mode: OA1\\[9:0\\]
contains the 10-bit own slave address. Note: These bits can be written only when OA1EN=0."]
    #[inline(always)]
    #[must_use]
    pub fn oa1(&mut self) -> OA1_W<0> {
        OA1_W::new(self)
    }
    #[doc = "Bit 10 - Own Address 1 10-bit mode Note: This bit can be written only when OA1EN=0."]
    #[inline(always)]
    #[must_use]
    pub fn oa1mode(&mut self) -> OA1MODE_W<10> {
        OA1MODE_W::new(self)
    }
    #[doc = "Bit 15 - Own Address 1 enable"]
    #[inline(always)]
    #[must_use]
    pub fn oa1en(&mut self) -> OA1EN_W<15> {
        OA1EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "I2C own address 1 register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [oar1](index.html) module"]
pub struct OAR1_SPEC;
impl crate::RegisterSpec for OAR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [oar1::R](R) reader structure"]
impl crate::Readable for OAR1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [oar1::W](W) writer structure"]
impl crate::Writable for OAR1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets OAR1 to value 0"]
impl crate::Resettable for OAR1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
