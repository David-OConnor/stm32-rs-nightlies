#[doc = "Register `DDRCTRL_ADDRMAP6` reader"]
pub struct R(crate::R<DDRCTRL_ADDRMAP6_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DDRCTRL_ADDRMAP6_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DDRCTRL_ADDRMAP6_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DDRCTRL_ADDRMAP6_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DDRCTRL_ADDRMAP6` writer"]
pub struct W(crate::W<DDRCTRL_ADDRMAP6_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DDRCTRL_ADDRMAP6_SPEC>;
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
impl From<crate::W<DDRCTRL_ADDRMAP6_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DDRCTRL_ADDRMAP6_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ADDRMAP_ROW_B12` reader - ADDRMAP_ROW_B12"]
pub type ADDRMAP_ROW_B12_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ADDRMAP_ROW_B12` writer - ADDRMAP_ROW_B12"]
pub type ADDRMAP_ROW_B12_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, DDRCTRL_ADDRMAP6_SPEC, u8, u8, 4, O>;
#[doc = "Field `ADDRMAP_ROW_B13` reader - ADDRMAP_ROW_B13"]
pub type ADDRMAP_ROW_B13_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ADDRMAP_ROW_B13` writer - ADDRMAP_ROW_B13"]
pub type ADDRMAP_ROW_B13_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, DDRCTRL_ADDRMAP6_SPEC, u8, u8, 4, O>;
#[doc = "Field `ADDRMAP_ROW_B14` reader - ADDRMAP_ROW_B14"]
pub type ADDRMAP_ROW_B14_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ADDRMAP_ROW_B14` writer - ADDRMAP_ROW_B14"]
pub type ADDRMAP_ROW_B14_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, DDRCTRL_ADDRMAP6_SPEC, u8, u8, 4, O>;
#[doc = "Field `ADDRMAP_ROW_B15` reader - ADDRMAP_ROW_B15"]
pub type ADDRMAP_ROW_B15_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ADDRMAP_ROW_B15` writer - ADDRMAP_ROW_B15"]
pub type ADDRMAP_ROW_B15_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, DDRCTRL_ADDRMAP6_SPEC, u8, u8, 4, O>;
#[doc = "Field `LPDDR3_6GB_12GB` reader - LPDDR3_6GB_12GB"]
pub type LPDDR3_6GB_12GB_R = crate::BitReader<bool>;
#[doc = "Field `LPDDR3_6GB_12GB` writer - LPDDR3_6GB_12GB"]
pub type LPDDR3_6GB_12GB_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, DDRCTRL_ADDRMAP6_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:3 - ADDRMAP_ROW_B12"]
    #[inline(always)]
    pub fn addrmap_row_b12(&self) -> ADDRMAP_ROW_B12_R {
        ADDRMAP_ROW_B12_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - ADDRMAP_ROW_B13"]
    #[inline(always)]
    pub fn addrmap_row_b13(&self) -> ADDRMAP_ROW_B13_R {
        ADDRMAP_ROW_B13_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - ADDRMAP_ROW_B14"]
    #[inline(always)]
    pub fn addrmap_row_b14(&self) -> ADDRMAP_ROW_B14_R {
        ADDRMAP_ROW_B14_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - ADDRMAP_ROW_B15"]
    #[inline(always)]
    pub fn addrmap_row_b15(&self) -> ADDRMAP_ROW_B15_R {
        ADDRMAP_ROW_B15_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bit 31 - LPDDR3_6GB_12GB"]
    #[inline(always)]
    pub fn lpddr3_6gb_12gb(&self) -> LPDDR3_6GB_12GB_R {
        LPDDR3_6GB_12GB_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - ADDRMAP_ROW_B12"]
    #[inline(always)]
    #[must_use]
    pub fn addrmap_row_b12(&mut self) -> ADDRMAP_ROW_B12_W<0> {
        ADDRMAP_ROW_B12_W::new(self)
    }
    #[doc = "Bits 8:11 - ADDRMAP_ROW_B13"]
    #[inline(always)]
    #[must_use]
    pub fn addrmap_row_b13(&mut self) -> ADDRMAP_ROW_B13_W<8> {
        ADDRMAP_ROW_B13_W::new(self)
    }
    #[doc = "Bits 16:19 - ADDRMAP_ROW_B14"]
    #[inline(always)]
    #[must_use]
    pub fn addrmap_row_b14(&mut self) -> ADDRMAP_ROW_B14_W<16> {
        ADDRMAP_ROW_B14_W::new(self)
    }
    #[doc = "Bits 24:27 - ADDRMAP_ROW_B15"]
    #[inline(always)]
    #[must_use]
    pub fn addrmap_row_b15(&mut self) -> ADDRMAP_ROW_B15_W<24> {
        ADDRMAP_ROW_B15_W::new(self)
    }
    #[doc = "Bit 31 - LPDDR3_6GB_12GB"]
    #[inline(always)]
    #[must_use]
    pub fn lpddr3_6gb_12gb(&mut self) -> LPDDR3_6GB_12GB_W<31> {
        LPDDR3_6GB_12GB_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DDRCTRL address register 6\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ddrctrl_addrmap6](index.html) module"]
pub struct DDRCTRL_ADDRMAP6_SPEC;
impl crate::RegisterSpec for DDRCTRL_ADDRMAP6_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ddrctrl_addrmap6::R](R) reader structure"]
impl crate::Readable for DDRCTRL_ADDRMAP6_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ddrctrl_addrmap6::W](W) writer structure"]
impl crate::Writable for DDRCTRL_ADDRMAP6_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DDRCTRL_ADDRMAP6 to value 0"]
impl crate::Resettable for DDRCTRL_ADDRMAP6_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
