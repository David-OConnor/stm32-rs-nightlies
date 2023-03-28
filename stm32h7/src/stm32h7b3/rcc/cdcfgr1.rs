#[doc = "Register `CDCFGR1` reader"]
pub struct R(crate::R<CDCFGR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CDCFGR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CDCFGR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CDCFGR1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CDCFGR1` writer"]
pub struct W(crate::W<CDCFGR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CDCFGR1_SPEC>;
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
impl From<crate::W<CDCFGR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CDCFGR1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `HPRE` reader - CPU domain AHB prescaler Set and reset by software to control the division factor of rcc_hclk3 and rcc_aclk. Changing this division ratio has an impact on the frequency of all bus matrix clocks. 0xxx: rcc_hclk3 = sys_cdcpre_ck (default after reset) Note: The clocks are divided by the new prescaler factor from1 to 16 periods of the slowest APB clock among rcc_pclk\\[4:1\\]
after HPRE update. Note: Note also that rcc_hclk3 = rcc_aclk."]
pub type HPRE_R = crate::FieldReader<u8, HPRE_A>;
#[doc = "CPU domain AHB prescaler Set and reset by software to control the division factor of rcc_hclk3 and rcc_aclk. Changing this division ratio has an impact on the frequency of all bus matrix clocks. 0xxx: rcc_hclk3 = sys_cdcpre_ck (default after reset) Note: The clocks are divided by the new prescaler factor from1 to 16 periods of the slowest APB clock among rcc_pclk\\[4:1\\]
after HPRE update. Note: Note also that rcc_hclk3 = rcc_aclk.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum HPRE_A {
    #[doc = "0: sys_ck not divided"]
    Div1 = 0,
    #[doc = "8: sys_ck divided by 2"]
    Div2 = 8,
    #[doc = "9: sys_ck divided by 4"]
    Div4 = 9,
    #[doc = "10: sys_ck divided by 8"]
    Div8 = 10,
    #[doc = "11: sys_ck divided by 16"]
    Div16 = 11,
    #[doc = "12: sys_ck divided by 64"]
    Div64 = 12,
    #[doc = "13: sys_ck divided by 128"]
    Div128 = 13,
    #[doc = "14: sys_ck divided by 256"]
    Div256 = 14,
    #[doc = "15: sys_ck divided by 512"]
    Div512 = 15,
}
impl From<HPRE_A> for u8 {
    #[inline(always)]
    fn from(variant: HPRE_A) -> Self {
        variant as _
    }
}
impl HPRE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<HPRE_A> {
        match self.bits {
            0 => Some(HPRE_A::Div1),
            8 => Some(HPRE_A::Div2),
            9 => Some(HPRE_A::Div4),
            10 => Some(HPRE_A::Div8),
            11 => Some(HPRE_A::Div16),
            12 => Some(HPRE_A::Div64),
            13 => Some(HPRE_A::Div128),
            14 => Some(HPRE_A::Div256),
            15 => Some(HPRE_A::Div512),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `Div1`"]
    #[inline(always)]
    pub fn is_div1(&self) -> bool {
        *self == HPRE_A::Div1
    }
    #[doc = "Checks if the value of the field is `Div2`"]
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        *self == HPRE_A::Div2
    }
    #[doc = "Checks if the value of the field is `Div4`"]
    #[inline(always)]
    pub fn is_div4(&self) -> bool {
        *self == HPRE_A::Div4
    }
    #[doc = "Checks if the value of the field is `Div8`"]
    #[inline(always)]
    pub fn is_div8(&self) -> bool {
        *self == HPRE_A::Div8
    }
    #[doc = "Checks if the value of the field is `Div16`"]
    #[inline(always)]
    pub fn is_div16(&self) -> bool {
        *self == HPRE_A::Div16
    }
    #[doc = "Checks if the value of the field is `Div64`"]
    #[inline(always)]
    pub fn is_div64(&self) -> bool {
        *self == HPRE_A::Div64
    }
    #[doc = "Checks if the value of the field is `Div128`"]
    #[inline(always)]
    pub fn is_div128(&self) -> bool {
        *self == HPRE_A::Div128
    }
    #[doc = "Checks if the value of the field is `Div256`"]
    #[inline(always)]
    pub fn is_div256(&self) -> bool {
        *self == HPRE_A::Div256
    }
    #[doc = "Checks if the value of the field is `Div512`"]
    #[inline(always)]
    pub fn is_div512(&self) -> bool {
        *self == HPRE_A::Div512
    }
}
#[doc = "Field `HPRE` writer - CPU domain AHB prescaler Set and reset by software to control the division factor of rcc_hclk3 and rcc_aclk. Changing this division ratio has an impact on the frequency of all bus matrix clocks. 0xxx: rcc_hclk3 = sys_cdcpre_ck (default after reset) Note: The clocks are divided by the new prescaler factor from1 to 16 periods of the slowest APB clock among rcc_pclk\\[4:1\\]
after HPRE update. Note: Note also that rcc_hclk3 = rcc_aclk."]
pub type HPRE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CDCFGR1_SPEC, u8, HPRE_A, 4, O>;
impl<'a, const O: u8> HPRE_W<'a, O> {
    #[doc = "sys_ck not divided"]
    #[inline(always)]
    pub fn div1(self) -> &'a mut W {
        self.variant(HPRE_A::Div1)
    }
    #[doc = "sys_ck divided by 2"]
    #[inline(always)]
    pub fn div2(self) -> &'a mut W {
        self.variant(HPRE_A::Div2)
    }
    #[doc = "sys_ck divided by 4"]
    #[inline(always)]
    pub fn div4(self) -> &'a mut W {
        self.variant(HPRE_A::Div4)
    }
    #[doc = "sys_ck divided by 8"]
    #[inline(always)]
    pub fn div8(self) -> &'a mut W {
        self.variant(HPRE_A::Div8)
    }
    #[doc = "sys_ck divided by 16"]
    #[inline(always)]
    pub fn div16(self) -> &'a mut W {
        self.variant(HPRE_A::Div16)
    }
    #[doc = "sys_ck divided by 64"]
    #[inline(always)]
    pub fn div64(self) -> &'a mut W {
        self.variant(HPRE_A::Div64)
    }
    #[doc = "sys_ck divided by 128"]
    #[inline(always)]
    pub fn div128(self) -> &'a mut W {
        self.variant(HPRE_A::Div128)
    }
    #[doc = "sys_ck divided by 256"]
    #[inline(always)]
    pub fn div256(self) -> &'a mut W {
        self.variant(HPRE_A::Div256)
    }
    #[doc = "sys_ck divided by 512"]
    #[inline(always)]
    pub fn div512(self) -> &'a mut W {
        self.variant(HPRE_A::Div512)
    }
}
#[doc = "Field `CDPPRE` reader - CPU domain APB3 prescaler Set and reset by software to control the division factor of rcc_pclk3. The clock is divided by the new prescaler factor from 1 to 16 cycles of rcc_hclk3 after CDPPRE write. 0xx: rcc_pclk3 = rcc_hclk3 (default after reset)"]
pub type CDPPRE_R = crate::FieldReader<u8, CDPPRE_A>;
#[doc = "CPU domain APB3 prescaler Set and reset by software to control the division factor of rcc_pclk3. The clock is divided by the new prescaler factor from 1 to 16 cycles of rcc_hclk3 after CDPPRE write. 0xx: rcc_pclk3 = rcc_hclk3 (default after reset)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CDPPRE_A {
    #[doc = "0: rcc_hclk not divided"]
    Div1 = 0,
    #[doc = "4: rcc_hclk divided by 2"]
    Div2 = 4,
    #[doc = "5: rcc_hclk divided by 4"]
    Div4 = 5,
    #[doc = "6: rcc_hclk divided by 8"]
    Div8 = 6,
    #[doc = "7: rcc_hclk divided by 16"]
    Div16 = 7,
}
impl From<CDPPRE_A> for u8 {
    #[inline(always)]
    fn from(variant: CDPPRE_A) -> Self {
        variant as _
    }
}
impl CDPPRE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<CDPPRE_A> {
        match self.bits {
            0 => Some(CDPPRE_A::Div1),
            4 => Some(CDPPRE_A::Div2),
            5 => Some(CDPPRE_A::Div4),
            6 => Some(CDPPRE_A::Div8),
            7 => Some(CDPPRE_A::Div16),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `Div1`"]
    #[inline(always)]
    pub fn is_div1(&self) -> bool {
        *self == CDPPRE_A::Div1
    }
    #[doc = "Checks if the value of the field is `Div2`"]
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        *self == CDPPRE_A::Div2
    }
    #[doc = "Checks if the value of the field is `Div4`"]
    #[inline(always)]
    pub fn is_div4(&self) -> bool {
        *self == CDPPRE_A::Div4
    }
    #[doc = "Checks if the value of the field is `Div8`"]
    #[inline(always)]
    pub fn is_div8(&self) -> bool {
        *self == CDPPRE_A::Div8
    }
    #[doc = "Checks if the value of the field is `Div16`"]
    #[inline(always)]
    pub fn is_div16(&self) -> bool {
        *self == CDPPRE_A::Div16
    }
}
#[doc = "Field `CDPPRE` writer - CPU domain APB3 prescaler Set and reset by software to control the division factor of rcc_pclk3. The clock is divided by the new prescaler factor from 1 to 16 cycles of rcc_hclk3 after CDPPRE write. 0xx: rcc_pclk3 = rcc_hclk3 (default after reset)"]
pub type CDPPRE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CDCFGR1_SPEC, u8, CDPPRE_A, 3, O>;
impl<'a, const O: u8> CDPPRE_W<'a, O> {
    #[doc = "rcc_hclk not divided"]
    #[inline(always)]
    pub fn div1(self) -> &'a mut W {
        self.variant(CDPPRE_A::Div1)
    }
    #[doc = "rcc_hclk divided by 2"]
    #[inline(always)]
    pub fn div2(self) -> &'a mut W {
        self.variant(CDPPRE_A::Div2)
    }
    #[doc = "rcc_hclk divided by 4"]
    #[inline(always)]
    pub fn div4(self) -> &'a mut W {
        self.variant(CDPPRE_A::Div4)
    }
    #[doc = "rcc_hclk divided by 8"]
    #[inline(always)]
    pub fn div8(self) -> &'a mut W {
        self.variant(CDPPRE_A::Div8)
    }
    #[doc = "rcc_hclk divided by 16"]
    #[inline(always)]
    pub fn div16(self) -> &'a mut W {
        self.variant(CDPPRE_A::Div16)
    }
}
#[doc = "Field `CDCPRE` reader - CPU domain core prescaler Set and reset by software to control the CPU domain CPU clock division factor. Changing this division ratio has an impact on the frequency of the CPU clock and all bus matrix clocks. After changing this prescaler value, it takes up to 16 periods of the slowest APB clock before the new division ratio is taken into account. The application can check if the new division factor is taken into account by reading back this register. 0xxx: sys_ck not divided (default after reset)"]
pub use HPRE_R as CDCPRE_R;
#[doc = "Field `CDCPRE` writer - CPU domain core prescaler Set and reset by software to control the CPU domain CPU clock division factor. Changing this division ratio has an impact on the frequency of the CPU clock and all bus matrix clocks. After changing this prescaler value, it takes up to 16 periods of the slowest APB clock before the new division ratio is taken into account. The application can check if the new division factor is taken into account by reading back this register. 0xxx: sys_ck not divided (default after reset)"]
pub use HPRE_W as CDCPRE_W;
impl R {
    #[doc = "Bits 0:3 - CPU domain AHB prescaler Set and reset by software to control the division factor of rcc_hclk3 and rcc_aclk. Changing this division ratio has an impact on the frequency of all bus matrix clocks. 0xxx: rcc_hclk3 = sys_cdcpre_ck (default after reset) Note: The clocks are divided by the new prescaler factor from1 to 16 periods of the slowest APB clock among rcc_pclk\\[4:1\\]
after HPRE update. Note: Note also that rcc_hclk3 = rcc_aclk."]
    #[inline(always)]
    pub fn hpre(&self) -> HPRE_R {
        HPRE_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:6 - CPU domain APB3 prescaler Set and reset by software to control the division factor of rcc_pclk3. The clock is divided by the new prescaler factor from 1 to 16 cycles of rcc_hclk3 after CDPPRE write. 0xx: rcc_pclk3 = rcc_hclk3 (default after reset)"]
    #[inline(always)]
    pub fn cdppre(&self) -> CDPPRE_R {
        CDPPRE_R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bits 8:11 - CPU domain core prescaler Set and reset by software to control the CPU domain CPU clock division factor. Changing this division ratio has an impact on the frequency of the CPU clock and all bus matrix clocks. After changing this prescaler value, it takes up to 16 periods of the slowest APB clock before the new division ratio is taken into account. The application can check if the new division factor is taken into account by reading back this register. 0xxx: sys_ck not divided (default after reset)"]
    #[inline(always)]
    pub fn cdcpre(&self) -> CDCPRE_R {
        CDCPRE_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - CPU domain AHB prescaler Set and reset by software to control the division factor of rcc_hclk3 and rcc_aclk. Changing this division ratio has an impact on the frequency of all bus matrix clocks. 0xxx: rcc_hclk3 = sys_cdcpre_ck (default after reset) Note: The clocks are divided by the new prescaler factor from1 to 16 periods of the slowest APB clock among rcc_pclk\\[4:1\\]
after HPRE update. Note: Note also that rcc_hclk3 = rcc_aclk."]
    #[inline(always)]
    #[must_use]
    pub fn hpre(&mut self) -> HPRE_W<0> {
        HPRE_W::new(self)
    }
    #[doc = "Bits 4:6 - CPU domain APB3 prescaler Set and reset by software to control the division factor of rcc_pclk3. The clock is divided by the new prescaler factor from 1 to 16 cycles of rcc_hclk3 after CDPPRE write. 0xx: rcc_pclk3 = rcc_hclk3 (default after reset)"]
    #[inline(always)]
    #[must_use]
    pub fn cdppre(&mut self) -> CDPPRE_W<4> {
        CDPPRE_W::new(self)
    }
    #[doc = "Bits 8:11 - CPU domain core prescaler Set and reset by software to control the CPU domain CPU clock division factor. Changing this division ratio has an impact on the frequency of the CPU clock and all bus matrix clocks. After changing this prescaler value, it takes up to 16 periods of the slowest APB clock before the new division ratio is taken into account. The application can check if the new division factor is taken into account by reading back this register. 0xxx: sys_ck not divided (default after reset)"]
    #[inline(always)]
    #[must_use]
    pub fn cdcpre(&mut self) -> CDCPRE_W<8> {
        CDCPRE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cdcfgr1](index.html) module"]
pub struct CDCFGR1_SPEC;
impl crate::RegisterSpec for CDCFGR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cdcfgr1::R](R) reader structure"]
impl crate::Readable for CDCFGR1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cdcfgr1::W](W) writer structure"]
impl crate::Writable for CDCFGR1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CDCFGR1 to value 0"]
impl crate::Resettable for CDCFGR1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
