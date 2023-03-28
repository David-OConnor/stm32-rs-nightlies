#[doc = "Register `I2C_OAR2` reader"]
pub struct R(crate::R<I2C_OAR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<I2C_OAR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<I2C_OAR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<I2C_OAR2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `I2C_OAR2` writer"]
pub struct W(crate::W<I2C_OAR2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<I2C_OAR2_SPEC>;
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
impl From<crate::W<I2C_OAR2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<I2C_OAR2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `OA2` reader - OA2"]
pub type OA2_R = crate::FieldReader<u8, u8>;
#[doc = "Field `OA2` writer - OA2"]
pub type OA2_W<'a, const O: u8> = crate::FieldWriter<'a, u32, I2C_OAR2_SPEC, u8, u8, 7, O>;
#[doc = "Field `OA2MSK` reader - OA2MSK"]
pub type OA2MSK_R = crate::FieldReader<u8, u8>;
#[doc = "Field `OA2MSK` writer - OA2MSK"]
pub type OA2MSK_W<'a, const O: u8> = crate::FieldWriter<'a, u32, I2C_OAR2_SPEC, u8, u8, 3, O>;
#[doc = "Field `OA2EN` reader - OA2EN"]
pub type OA2EN_R = crate::BitReader<bool>;
#[doc = "Field `OA2EN` writer - OA2EN"]
pub type OA2EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, I2C_OAR2_SPEC, bool, O>;
impl R {
    #[doc = "Bits 1:7 - OA2"]
    #[inline(always)]
    pub fn oa2(&self) -> OA2_R {
        OA2_R::new(((self.bits >> 1) & 0x7f) as u8)
    }
    #[doc = "Bits 8:10 - OA2MSK"]
    #[inline(always)]
    pub fn oa2msk(&self) -> OA2MSK_R {
        OA2MSK_R::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bit 15 - OA2EN"]
    #[inline(always)]
    pub fn oa2en(&self) -> OA2EN_R {
        OA2EN_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 1:7 - OA2"]
    #[inline(always)]
    #[must_use]
    pub fn oa2(&mut self) -> OA2_W<1> {
        OA2_W::new(self)
    }
    #[doc = "Bits 8:10 - OA2MSK"]
    #[inline(always)]
    #[must_use]
    pub fn oa2msk(&mut self) -> OA2MSK_W<8> {
        OA2MSK_W::new(self)
    }
    #[doc = "Bit 15 - OA2EN"]
    #[inline(always)]
    #[must_use]
    pub fn oa2en(&mut self) -> OA2EN_W<15> {
        OA2EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Access: No wait states, except if a write access occurs while a write access to this register is ongoing. In this case, wait states are inserted in the second write access until the previous one is completed. The latency of the second write access can be up to 2 x i2c_pclk + 6 x i2c_ker_ck.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [i2c_oar2](index.html) module"]
pub struct I2C_OAR2_SPEC;
impl crate::RegisterSpec for I2C_OAR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [i2c_oar2::R](R) reader structure"]
impl crate::Readable for I2C_OAR2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [i2c_oar2::W](W) writer structure"]
impl crate::Writable for I2C_OAR2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets I2C_OAR2 to value 0"]
impl crate::Resettable for I2C_OAR2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
