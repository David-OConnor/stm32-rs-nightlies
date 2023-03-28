#[doc = "Register `DDRCTRL_ADDRMAP5` reader"]
pub struct R(crate::R<DDRCTRL_ADDRMAP5_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DDRCTRL_ADDRMAP5_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DDRCTRL_ADDRMAP5_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DDRCTRL_ADDRMAP5_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DDRCTRL_ADDRMAP5` writer"]
pub struct W(crate::W<DDRCTRL_ADDRMAP5_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DDRCTRL_ADDRMAP5_SPEC>;
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
impl From<crate::W<DDRCTRL_ADDRMAP5_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DDRCTRL_ADDRMAP5_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ADDRMAP_ROW_B0` reader - ADDRMAP_ROW_B0"]
pub type ADDRMAP_ROW_B0_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ADDRMAP_ROW_B0` writer - ADDRMAP_ROW_B0"]
pub type ADDRMAP_ROW_B0_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, DDRCTRL_ADDRMAP5_SPEC, u8, u8, 4, O>;
#[doc = "Field `ADDRMAP_ROW_B1` reader - ADDRMAP_ROW_B1"]
pub type ADDRMAP_ROW_B1_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ADDRMAP_ROW_B1` writer - ADDRMAP_ROW_B1"]
pub type ADDRMAP_ROW_B1_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, DDRCTRL_ADDRMAP5_SPEC, u8, u8, 4, O>;
#[doc = "Field `ADDRMAP_ROW_B2_10` reader - ADDRMAP_ROW_B2_10"]
pub type ADDRMAP_ROW_B2_10_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ADDRMAP_ROW_B2_10` writer - ADDRMAP_ROW_B2_10"]
pub type ADDRMAP_ROW_B2_10_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, DDRCTRL_ADDRMAP5_SPEC, u8, u8, 4, O>;
#[doc = "Field `ADDRMAP_ROW_B11` reader - ADDRMAP_ROW_B11"]
pub type ADDRMAP_ROW_B11_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ADDRMAP_ROW_B11` writer - ADDRMAP_ROW_B11"]
pub type ADDRMAP_ROW_B11_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, DDRCTRL_ADDRMAP5_SPEC, u8, u8, 4, O>;
impl R {
    #[doc = "Bits 0:3 - ADDRMAP_ROW_B0"]
    #[inline(always)]
    pub fn addrmap_row_b0(&self) -> ADDRMAP_ROW_B0_R {
        ADDRMAP_ROW_B0_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - ADDRMAP_ROW_B1"]
    #[inline(always)]
    pub fn addrmap_row_b1(&self) -> ADDRMAP_ROW_B1_R {
        ADDRMAP_ROW_B1_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - ADDRMAP_ROW_B2_10"]
    #[inline(always)]
    pub fn addrmap_row_b2_10(&self) -> ADDRMAP_ROW_B2_10_R {
        ADDRMAP_ROW_B2_10_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - ADDRMAP_ROW_B11"]
    #[inline(always)]
    pub fn addrmap_row_b11(&self) -> ADDRMAP_ROW_B11_R {
        ADDRMAP_ROW_B11_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - ADDRMAP_ROW_B0"]
    #[inline(always)]
    #[must_use]
    pub fn addrmap_row_b0(&mut self) -> ADDRMAP_ROW_B0_W<0> {
        ADDRMAP_ROW_B0_W::new(self)
    }
    #[doc = "Bits 8:11 - ADDRMAP_ROW_B1"]
    #[inline(always)]
    #[must_use]
    pub fn addrmap_row_b1(&mut self) -> ADDRMAP_ROW_B1_W<8> {
        ADDRMAP_ROW_B1_W::new(self)
    }
    #[doc = "Bits 16:19 - ADDRMAP_ROW_B2_10"]
    #[inline(always)]
    #[must_use]
    pub fn addrmap_row_b2_10(&mut self) -> ADDRMAP_ROW_B2_10_W<16> {
        ADDRMAP_ROW_B2_10_W::new(self)
    }
    #[doc = "Bits 24:27 - ADDRMAP_ROW_B11"]
    #[inline(always)]
    #[must_use]
    pub fn addrmap_row_b11(&mut self) -> ADDRMAP_ROW_B11_W<24> {
        ADDRMAP_ROW_B11_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DDRCTRL address map register 5\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ddrctrl_addrmap5](index.html) module"]
pub struct DDRCTRL_ADDRMAP5_SPEC;
impl crate::RegisterSpec for DDRCTRL_ADDRMAP5_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ddrctrl_addrmap5::R](R) reader structure"]
impl crate::Readable for DDRCTRL_ADDRMAP5_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ddrctrl_addrmap5::W](W) writer structure"]
impl crate::Writable for DDRCTRL_ADDRMAP5_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DDRCTRL_ADDRMAP5 to value 0"]
impl crate::Resettable for DDRCTRL_ADDRMAP5_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
